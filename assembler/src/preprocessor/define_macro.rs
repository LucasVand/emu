use crate::preprocessor::preprocessor_error::PreprocessorError;
use crate::preprocessor::preprocessor_error::PreprocessorErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use std::vec;
use std::{fmt::Display, iter::Peekable};

#[derive(Debug, Clone)]
pub struct DefineMacro {
    pub label: String,
    pub value: String,
    pub value_kind: TokenType,
}
impl PartialEq for DefineMacro {
    fn eq(&self, other: &Self) -> bool {
        return self.label == other.label;
    }
}

impl DefineMacro {
    pub fn new(label: &str, value: &str, kind: TokenType) -> Self {
        DefineMacro {
            label: label.to_string(),
            value: value.to_string(),
            value_kind: kind,
        }
    }

    pub fn replace_defines(tokens: Vec<Token>) -> (Vec<Token>, Vec<PreprocessorError>) {
        let mut errors: Vec<PreprocessorError> = Vec::new();
        let mut new_tokens_list: Vec<Token> = Vec::new();
        // create a consuming iterator
        let mut iter = tokens.into_iter().peekable();
        let mut define_list: Vec<DefineMacro> = Vec::new();

        while let Some(current) = iter.next() {
            if current.kind == TokenType::DefineKeyword {
                // we have a define and we need to add it
                let macro_def = Self::create_definition(&mut iter, &current);
                match macro_def {
                    Ok(def) => {
                        // check for duplicates
                        if define_list.contains(&def) {
                            errors.push(PreprocessorError::new(
                                current.token_info.clone(),
                                PreprocessorErrorType::DuplicateDefinitionsFound,
                            ));
                        } else {
                            define_list.push(def);
                        }
                    }
                    Err(err) => errors.push(err),
                };
            } else if current.kind == TokenType::Label {
                // get the new token
                let new_token = Self::sub_label(current, &define_list);
                match new_token {
                    Ok(new_token) => new_tokens_list.push(new_token),
                    Err(err) => errors.push(err),
                }
            } else if current.kind == TokenType::UnDefineKeyword {
                let res = Self::remove_definition(&mut iter, &mut define_list, &current);
                if let Err(err) = res {
                    errors.push(err);
                }
            } else if current.kind == TokenType::Expression {
                let new_token = Self::sub_expression(current, &define_list);
                new_tokens_list.push(new_token);
            } else {
                new_tokens_list.push(current);
            }
        }
        return (new_tokens_list, errors);
    }
    pub fn sub_expression(token: Token, define_macros: &Vec<DefineMacro>) -> Token {
        let label_exists = define_macros.iter().find(|def| {
            let stripped_token = &token.token;

            return stripped_token.contains(&def.label);
        });

        if label_exists.is_none() {
            return token;
        }

        let label_def = label_exists.unwrap();

        let new_token = token.token.replace(&label_def.label, &label_def.value);

        return Token::new(new_token, token.kind, token.token_info);
    }
    pub fn sub_label(
        token: Token,
        define_macros: &Vec<DefineMacro>,
    ) -> Result<Token, PreprocessorError> {
        let label_exists = define_macros.iter().find(|def| {
            let stripped_token = token
                .token
                .strip_prefix("[")
                .unwrap_or(&token.token)
                .strip_suffix("]")
                .unwrap_or(&token.token);

            return def.label == stripped_token;
        });

        // if the label doesn not exist it might still be
        // a address label so dont throw error
        if label_exists.is_none() {
            return Ok(token);
        }
        let definition = label_exists.unwrap();

        let new_value = if token.is_addr {
            format!("[{}]", &definition.value)
        } else {
            definition.value.clone()
        };

        let info = token.token_info;
        let new_token = if token.is_addr {
            Token::new_address(new_value, definition.value_kind.clone(), info)
        } else {
            Token::new(new_value, definition.value_kind.clone(), info)
        };

        return Ok(new_token);
    }
    fn remove_definition<'a>(
        iter: &mut Peekable<vec::IntoIter<Token>>,
        define_list: &mut Vec<DefineMacro>,
        inital_token: &Token,
    ) -> Result<(), PreprocessorError> {
        let label = iter.peek();
        if label.is_none() {
            return Err(PreprocessorError::new(
                inital_token.token_info.clone(),
                PreprocessorErrorType::ExpectedDefineLabel,
            ));
        }

        let label = label.unwrap();

        if label.kind != TokenType::Label {
            return Err(PreprocessorError::new(
                inital_token.token_info.clone(),
                PreprocessorErrorType::ExpectedDefineLabel,
            ));
        }
        let label = iter.next().unwrap();
        let index = define_list.iter().position(|define| {
            return define.label == label.token;
        });
        if index.is_none() {
            return Err(PreprocessorError::new(
                inital_token.token_info.clone(),
                PreprocessorErrorType::UnableToFindDefinitionToRemove,
            ));
        }

        let index = index.unwrap();

        define_list.swap_remove(index);

        return Ok(());
    }

    fn create_definition<'a>(
        iter: &mut Peekable<vec::IntoIter<Token>>,
        inital_token: &Token,
    ) -> Result<DefineMacro, PreprocessorError> {
        // peek at the next token
        let label = iter.peek();

        if label.is_none() {
            return Err(PreprocessorError::new(
                inital_token.token_info.clone(),
                PreprocessorErrorType::ExpectedDefineLabel,
            ));
        }
        let label = label.unwrap();
        if label.kind != TokenType::DefineDefinitionLabel {
            return Err(PreprocessorError::new(
                inital_token.token_info.clone(),
                PreprocessorErrorType::ExpectedDefineLabel,
            ));
        }

        // grab the next token
        let label = iter.next().unwrap();

        // peek at the value
        let value = iter.peek();

        if value.is_none() {
            return Err(PreprocessorError::new(
                inital_token.token_info.clone(),
                PreprocessorErrorType::ExpectedDefineValue,
            ));
        }

        // grab the value
        let value = iter.next().unwrap();

        let macro_def = DefineMacro::new(&label.token, &value.token, value.kind.clone());

        return Ok(macro_def);
    }
}
impl Display for DefineMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
