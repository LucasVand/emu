use crate::utils::token::Token;
use crate::utils::token::TokenType;
use std::iter::Peekable;

use crate::{
    compile::{compiled_token::CompiledToken, parse_literal::ParseLiteral},
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

        Logging::log_compiler_error_info("expected a data definition", &inital_token.token_info);
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
                    compiled.push(CompiledToken::create_token(lit, &ele.token_info));
                }

                // look at the last added byte
                if let Some(last) = compiled.last() {
                    match last {
                        // if the byte is not a null terminator add one
                        CompiledToken::Binary { byte, info } => {
                            let ch = *byte as char;
                            if ch != '\0' {
                                compiled.push(CompiledToken::create_token('\0' as u8, &info));
                            }
                        }
                        CompiledToken::Label { name: _, info: _ } => {
                            panic!(
                                "This should never happen, we cannot compile a string into a label"
                            );
                        }
                    }
                }
            } else {
                Logging::log_compiler_error_info("expected a string", &ele.token_info);
            }
        }
    }
    fn compile_double_word(compiled: &mut Vec<CompiledToken>, data_items: &Vec<Token>) {
        for ele in data_items {
            if TokenType::Label == ele.kind {
                compiled.push(CompiledToken::create_label(&ele.token, &ele.token_info));
            } else if TokenType::LITERALS.contains(&ele.kind)
                && ![TokenType::String].contains(&ele.kind)
            {
                let doubleword = ParseLiteral::parse_u16(&ele);
                let byte3 = doubleword as u8;
                let byte2 = (doubleword >> 8) as u8;

                compiled.push(CompiledToken::create_token(byte2, &ele.token_info));
                compiled.push(CompiledToken::create_token(byte3, &ele.token_info));
            } else {
                Logging::log_compiler_error_info(
                    "unable to parse double word this data item",
                    &ele.token_info,
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
                compiled.push(CompiledToken::create_token(byte, &ele.token_info));
            } else {
                Logging::log_compiler_error_info(
                    "unable to parse this word data item",
                    &ele.token_info,
                );
            }
        }
    }
}
