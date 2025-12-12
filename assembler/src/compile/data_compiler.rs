use std::iter::Peekable;

use crate::{
    compile::{compiled_token::CompiledToken, parse_literal::ParseLiteral},
    lex::token::{Token, TokenType},
    utils::logging::Logging,
};

pub struct DataCompiler {}

impl DataCompiler {
    pub fn compile_data<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
        inital_token: &Token,
        compiled: &mut Vec<CompiledToken>,
    ) {
        let mut data_items: Vec<Token> = Vec::new();

        while let Some(curr) = iter.peek() {
            if TokenType::LITERALS.contains(&curr.kind) {
                data_items.push(iter.next().unwrap().clone());
            } else {
                break;
            }
        }

        if inital_token.kind == TokenType::WordDataDefineKeyword {
            Self::compile_word(compiled, &data_items);
            return;
        }
        if inital_token.kind == TokenType::DoubleWordDataDefineKeyword {
            Self::compile_double_word(compiled, &data_items);
            return;
        }
        if inital_token.kind == TokenType::StringDataDefineKeyword {
            Self::compile_string(compiled, &data_items);
            return;
        }

        Logging::log_compiler_error_specific(
            "expected a data definition",
            inital_token.line_num,
            &inital_token.line,
            &inital_token.token,
        );
    }
    fn compile_string(compiled: &mut Vec<CompiledToken>, data_items: &Vec<Token>) {
        for ele in data_items {
            if TokenType::String == ele.kind {
                let quotes = &ele.token;
                let remove_quotes = quotes
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap();
                for ch in remove_quotes.chars() {
                    let lit = ch as u8;
                    compiled.push(CompiledToken::create_token(lit));
                }

                // look at the last added byte
                if let Some(last) = compiled.last() {
                    match last {
                        // if the byte is not a null terminator add one
                        CompiledToken::Binary { byte } => {
                            let ch = *byte as char;
                            if ch != '\0' {
                                compiled.push(CompiledToken::create_token('\0' as u8));
                            }
                        }
                        CompiledToken::Label { name: _ } => {
                            panic!(
                                "This should never happen, we cannot compile a string into a label"
                            );
                        }
                    }
                }
            } else {
                Logging::log_compiler_error_specific(
                    "expected a string",
                    ele.line_num,
                    &ele.line,
                    &ele.token,
                );
            }
        }
    }
    fn compile_double_word(compiled: &mut Vec<CompiledToken>, data_items: &Vec<Token>) {
        for ele in data_items {
            if TokenType::Label == ele.kind {
                compiled.push(CompiledToken::create_label(&ele.token));
            } else if TokenType::LITERALS.contains(&ele.kind)
                && ![TokenType::String].contains(&ele.kind)
            {
                let doubleword = ParseLiteral::parse_u16(&ele);
                let byte3 = doubleword as u8;
                let byte2 = (doubleword >> 8) as u8;

                compiled.push(CompiledToken::create_token(byte2));
                compiled.push(CompiledToken::create_token(byte3));
            } else {
                Logging::log_compiler_error_specific(
                    "unable to parse double word this data item",
                    ele.line_num,
                    &ele.line,
                    &ele.token,
                );
            }
        }
    }
    fn compile_word(compiled: &mut Vec<CompiledToken>, data_items: &Vec<Token>) {
        for ele in data_items {
            if TokenType::LITERALS.contains(&ele.kind)
                && ![TokenType::Label, TokenType::String].contains(&ele.kind)
            {
                let byte = ParseLiteral::parse_u8(ele);
                compiled.push(CompiledToken::create_token(byte));
            } else {
                Logging::log_compiler_error_specific(
                    "unable to parse this word data item",
                    ele.line_num,
                    &ele.line,
                    &ele.token,
                );
            }
        }
    }
}
