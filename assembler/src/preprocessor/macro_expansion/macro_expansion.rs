use crate::preprocessor::macro_expansion::macro_definition::MacroDefinition;
use crate::utils::logging::Logging;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use std::{fmt::Display, iter::Peekable};

#[derive(Debug)]
pub struct MacroExpansion {
    pub index: usize,
    pub parameter_count: usize,
    pub tokens: Vec<Token>,
}
impl MacroExpansion {
    pub fn new(index: usize, param_count: usize) -> MacroExpansion {
        MacroExpansion {
            parameter_count: param_count,
            index: index,
            tokens: Vec::new(),
        }
    }
    pub fn expand_macro<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
        inital_token: &Token,
        macro_list: &Vec<MacroDefinition>,
        index: &mut usize,
    ) -> Option<MacroExpansion> {
        let inital_index: usize = *index;
        // find the macro definition for the macro token found
        let macro_def = macro_list.iter().find(|macro_def| {
            return macro_def.label == inital_token.token;
        });

        // if we cant find it log error and return
        if macro_def.is_none() {
            Logging::log_preprocessor_error_info(
                "unable to find macro definition",
                &inital_token.token_info,
            );
            return None;
        }
        // we know it exists unwrap it
        let macro_def = macro_def.unwrap();

        let mut parameter_list: Vec<Token> = Vec::new();

        // while the next value is a parameter insert into the list
        while let Some(current) = iter.peek() {
            if TokenType::INSTRUCTION_OPERANDS.contains(&current.kind) {
                parameter_list.push(iter.next().unwrap().clone());
                *index += 1;
            } else {
                // as soon as we find one thats not break
                break;
            }
        }

        // if we have an incorrect number of parameters
        if parameter_list.len() != macro_def.parameters.len() {
            Logging::log_preprocessor_error_info(
                &format!(
                    "incorrect number of parameters, expected {} found {}",
                    macro_def.parameters.len(),
                    parameter_list.len()
                ),
                &inital_token.token_info,
            );
            return None;
        }

        // create the expansion
        let mut exp = MacroExpansion::new(inital_index, parameter_list.len());
        exp.tokens = macro_def.value.clone();

        let mut is_valid = true;
        // loop over all the tokens
        exp.tokens.iter_mut().for_each(|def_token| {
            // if token is expression
            if def_token.kind == TokenType::Expression {
                for index in 0..macro_def.parameters.len() {
                    let param = &macro_def.parameters[index];
                    def_token.token = def_token
                        .token
                        .replace(&param.token, &parameter_list[index].token);
                }
            }
            // if the token is a parameter
            if def_token.kind == TokenType::MacroParameter {
                // find the token to swap with
                let index = macro_def.parameters.iter().position(|param| {
                    return param.token == def_token.token;
                });
                // if we cannot find the index then incorrect arguments
                if index.is_none() {
                    Logging::log_preprocessor_error_info(
                        "unable to find argument",
                        &def_token.token_info,
                    );
                    is_valid = false;
                    return;
                }

                // get the param
                let param = &parameter_list[index.unwrap()];

                // swap the token with the param
                *def_token = param.clone();
            }
        });

        if !is_valid {
            return None;
        }

        return Some(exp);
    }
}
impl Display for MacroExpansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
