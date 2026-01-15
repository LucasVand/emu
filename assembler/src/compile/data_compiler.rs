use crate::compile::compile_error::CompilerError;
use crate::compile::compile_error::CompilerErrorType;
use crate::compile::expression_compiler::expression_solver::ExpressionSolver;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use std::iter::Peekable;
use std::vec;

use crate::compile::{compiled_token::CompiledToken, parse_literal::ParseLiteral};

pub struct DataCompiler {}

impl DataCompiler {
    pub fn compile_data(
        iter: &mut Peekable<vec::IntoIter<Token>>,
        inital_token: Token,
    ) -> (Vec<CompiledToken>, Vec<CompilerError>) {
        let mut data_items: Vec<Token> = Vec::new();

        while let Some(curr) = iter.peek() {
            if TokenType::LITERALS.contains(&curr.kind) {
                data_items.push(iter.next().unwrap().clone());
            } else {
                break;
            }
        }

        if inital_token.kind == TokenType::WordDataDefineKeyword {
            return Self::compile_word(data_items);
        }
        if inital_token.kind == TokenType::DoubleWordDataDefineKeyword {
            return Self::compile_double_word(data_items);
        }
        if inital_token.kind == TokenType::StringDataDefineKeyword {
            return Self::compile_string(data_items);
        }
        if inital_token.kind == TokenType::SpaceDataDefineKeyword {
            return Self::compile_space(data_items);
        }
        return (
            Vec::new(),
            vec![CompilerError::new(
                inital_token.token_info,
                CompilerErrorType::ExpectedDataDefinition,
            )],
        );
    }

    fn compile_space(data_items: Vec<Token>) -> (Vec<CompiledToken>, Vec<CompilerError>) {
        let mut size = 0;
        let mut compiled = Vec::new();
        let info = data_items[0].token_info.clone();

        for item in data_items {
            let lit;
            if item.kind == TokenType::Expression {
                lit = ExpressionSolver::solve(&item) as usize as u16;
            } else {
                lit = ParseLiteral::parse_u16(&item);
            }
            size += lit;
        }

        for _ in 0..size {
            compiled.push(CompiledToken::create_word(0, info.clone()));
        }
        return (compiled, Vec::new());
    }
    fn compile_string(data_items: Vec<Token>) -> (Vec<CompiledToken>, Vec<CompilerError>) {
        let mut compiled = Vec::new();
        let mut error_list = Vec::new();

        for ele in data_items {
            if TokenType::String == ele.kind {
                let quotes = &ele.token;
                let remove_quotes = quotes
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap();
                for ch in remove_quotes.chars() {
                    let mut info = ele.token_info.clone();
                    info.token = ch.to_string();
                    if ch as usize > 255 {
                        error_list.push(CompilerError::new(
                            info,
                            CompilerErrorType::CharacterIsNotValidASCII,
                        ))
                    } else {
                        let lit = ch as u8;
                        compiled.push(CompiledToken::create_word(lit, info));
                    }
                }

                // look at the last added byte
                if let Some(last) = compiled.last() {
                    if let CompiledToken::Binary { byte, info } = last {
                        let ch = *byte as char;
                        if ch != '\0' {
                            compiled.push(CompiledToken::create_word('\0' as u8, info.clone()));
                        }
                    }
                }
            } else {
                error_list.push(CompilerError::new(
                    ele.token_info,
                    CompilerErrorType::ExpectedString,
                ));
            }
        }
        return (compiled, error_list);
    }
    fn compile_double_word(data_items: Vec<Token>) -> (Vec<CompiledToken>, Vec<CompilerError>) {
        let mut compiled = Vec::new();
        let mut error_list = Vec::new();
        for ele in data_items {
            if TokenType::Label == ele.kind {
                compiled.push(CompiledToken::create_label(ele.token, ele.token_info));
            } else if TokenType::LITERALS.contains(&ele.kind)
                && ![TokenType::String].contains(&ele.kind)
            {
                if TokenType::Expression == ele.kind {
                    compiled.push(CompiledToken::create_expression(
                        ele.token,
                        true,
                        ele.token_info,
                    ));
                } else {
                    let doubleword = ParseLiteral::parse_u16(&ele);
                    compiled.push(CompiledToken::create_double_word(
                        doubleword,
                        ele.token_info,
                    ));
                }
            } else {
                error_list.push(CompilerError::new(
                    ele.token_info,
                    CompilerErrorType::CannotParseIntoWord,
                ));
            }
        }
        return (compiled, error_list);
    }
    fn compile_word(data_items: Vec<Token>) -> (Vec<CompiledToken>, Vec<CompilerError>) {
        let mut compiled = Vec::new();
        let mut errors = Vec::new();

        for ele in data_items {
            // is a literal but not a string or label
            if TokenType::LITERALS.contains(&ele.kind)
                && ![TokenType::Label, TokenType::String].contains(&ele.kind)
            {
                if TokenType::Expression == ele.kind {
                    compiled.push(CompiledToken::create_expression(
                        ele.token,
                        false,
                        ele.token_info,
                    ));
                } else {
                    let byte = ParseLiteral::parse_u8(&ele);
                    compiled.push(CompiledToken::create_word(byte, ele.token_info));
                }
            } else {
                errors.push(CompilerError::new(
                    ele.token_info,
                    CompilerErrorType::CannotParseIntoWord,
                ));
            }
        }
        return (compiled, errors);
    }
}
