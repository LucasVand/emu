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
        let mut new_token_list: Vec<Token> = Vec::new();

        let operand_list = Self::find_parameters(iter);

        let macro_def = Self::find_macro_definition(&inital_token, &operand_list, &macro_list)?;

        // create the expansion
        let mut expansion_tokens = macro_def.value.clone().into_iter().peekable();

        while let Some(token) = expansion_tokens.next() {
            match token.kind {
                TokenType::Expression => {
                    let mut new_token = token;
                    for index in 0..macro_def.parameters.len() {
                        let param = &macro_def.parameters[index];
                        new_token.token = new_token
                            .token
                            .replace(&param.token, &operand_list[index].token);
                    }
                    new_token_list.push(new_token);
                }
                TokenType::MacroParameter => {
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
                    let param = &operand_list[index.unwrap()];

                    new_token_list.push(param.clone());
                }
                _ => new_token_list.push(token),
            }
        }
        // check for nested macros
        let mut expansion_tokens = new_token_list.into_iter().peekable();
        let mut new_token_list = Vec::new();
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
            } else {
                new_token_list.push(token);
            }
        }
        return Ok(new_token_list);
    }
    fn find_parameters(iter: &mut Peekable<vec::IntoIter<Token>>) -> Vec<Token> {
        let mut parameter_list: Vec<Token> = Vec::new();

        // while the next value is a parameter insert into the list
        while let Some(current) = iter.peek() {
            if TokenType::INSTRUCTION_OPERANDS.contains(&current.kind) {
                parameter_list.push(iter.next().unwrap());
            } else {
                // as soon as we find one thats not break
                break;
            }
        }
        return parameter_list;
    }
    fn find_macro_definition(
        inital_token: &Token,
        operands: &Vec<Token>,
        def_list: &Vec<MacroDefinition>,
    ) -> Result<MacroDefinition, PreprocessorError> {
        let mut def_iter = def_list.iter();

        while let Some(def) = def_iter.next() {
            // same name
            let same_name = def.label.to_lowercase() == inital_token.token.to_lowercase();
            if !same_name {
                continue;
            }
            let res = Self::check_parameters(inital_token, &def.parameters, operands);
            if res.is_err() {
                continue;
            }

            return Ok(def.clone());
        }

        // we know we didnt find one down here
        // see if we matched a name
        let name_match = def_list.iter().find(|macro_def| {
            return macro_def.label.to_lowercase() == inital_token.token.to_lowercase();
        });
        // if we matched a name
        if name_match.is_some() {
            Self::check_parameters(inital_token, &name_match.unwrap().parameters, operands)?
        }
        // else we couldnt find the macro
        return Err(PreprocessorError::new(
            inital_token.token_info.clone(),
            PreprocessorErrorType::new_unable_to_find_macro_definition(
                &inital_token.token,
                def_list,
            ),
        ));
    }
    fn check_parameters(
        inital_token: &Token,
        params: &Vec<Token>,
        operands: &Vec<Token>,
    ) -> Result<(), PreprocessorError> {
        // same param size
        if operands.len() != params.len() {
            return Err(PreprocessorError::new(
                inital_token.token_info.clone(),
                PreprocessorErrorType::IncorrectNumberOfOperands {
                    expected: params.len(),
                    found: operands.len(),
                },
            ));
        }

        // loop over all params and check them
        for index in 0..operands.len() {
            let operand = &operands[index];
            let def_param = &params[index];

            let typed_operand = TypedMacroParameter::type_inst_parameter(operand);
            let typed_param = TypedMacroParameter::type_macro_parameter(def_param);

            if !typed_operand.is_equal(&typed_param) {
                return Err(PreprocessorError::new(
                    operand.token_info.clone(),
                    PreprocessorErrorType::IncorrectMacroOperands {
                        expected: typed_param,
                        found: typed_operand,
                    },
                ));
            }
        }
        return Ok(());
    }
}
impl Display for MacroExpansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
