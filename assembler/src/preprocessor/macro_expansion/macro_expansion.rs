use crate::preprocessor::macro_expansion::macro_definition::MacroDefinition;
use crate::preprocessor::macro_expansion::macro_parameters::TypedMacroParameter;
use crate::preprocessor::preprocessor_error::PreprocessorError;
use crate::preprocessor::preprocessor_error::PreprocessorErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use std::vec;
use std::{fmt::Display, iter::Peekable};

#[derive(Debug)]
pub struct MacroExpansion {}
impl MacroExpansion {
    pub fn expand_macro(
        iter: &mut Peekable<vec::IntoIter<Token>>,
        inital_token: Token,
        macro_list: &Vec<MacroDefinition>,
    ) -> Result<Vec<Token>, PreprocessorError> {
        let mut parameter_list: Vec<Token> = Vec::new();
        let mut new_token_list: Vec<Token> = Vec::new();

        // while the next value is a parameter insert into the list
        while let Some(current) = iter.peek() {
            if TokenType::INSTRUCTION_OPERANDS.contains(&current.kind) {
                parameter_list.push(iter.next().unwrap());
            } else {
                // as soon as we find one thats not break
                break;
            }
        }

        let macro_def = macro_list.iter().find(|macro_def| {
            // make sure the name is the same
            let same_name = macro_def.label == inital_token.token;
            if !same_name {
                return false;
            }
            // same param size
            if parameter_list.len() != macro_def.parameters.len() {
                return false;
            }
            // type all the params and check them
            for (index, param) in parameter_list.iter().enumerate() {
                let typed_inst = TypedMacroParameter::type_inst_parameter(param);
                let typed_param =
                    TypedMacroParameter::type_macro_parameter(&macro_def.parameters[index]);
                if !typed_inst.is_equal(typed_param) {
                    return false;
                }
            }
            return true;
        });

        // if we cant find it log error and return
        if macro_def.is_none() {
            return Err(PreprocessorError::new(
                inital_token.token_info.clone(),
                PreprocessorErrorType::UnableToFindMacroDefinition,
            ));
        }
        // we know it exists unwrap it
        let macro_def = macro_def.unwrap();

        // create the expansion
        let mut expansion_tokens = macro_def.value.clone().into_iter().peekable();

        while let Some(token) = expansion_tokens.next() {
            if token.kind == TokenType::MacroMnemonic {
                if token.token == macro_def.label {
                    return Err(PreprocessorError::new(
                        token.token_info,
                        PreprocessorErrorType::InfiniteRecursionMacro,
                    ));
                } else {
                    let mut expansion =
                        Self::expand_macro(&mut expansion_tokens, token, &macro_list)?;
                    new_token_list.append(&mut expansion)
                }
            } else if token.kind == TokenType::Expression {
                let mut new_token = token;
                for index in 0..macro_def.parameters.len() {
                    let param = &macro_def.parameters[index];
                    new_token.token = new_token
                        .token
                        .replace(&param.token, &parameter_list[index].token);
                }
                new_token_list.push(new_token);
            } else if token.kind == TokenType::MacroParameter {
                // find the token to swap with
                let index = macro_def.parameters.iter().position(|param| {
                    return param.token == token.token;
                });
                // if we cannot find the index then incorrect arguments
                if index.is_none() {
                    return Err(PreprocessorError::new(
                        token.token_info,
                        PreprocessorErrorType::UnableToFindMacroParameter,
                    ));
                }

                // get the param
                let param = &parameter_list[index.unwrap()];

                new_token_list.push(param.clone());
            } else {
                new_token_list.push(token);
            }
        }
        return Ok(new_token_list);
    }
}
impl Display for MacroExpansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
