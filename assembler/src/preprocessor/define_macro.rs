use crate::utils::logging::Logging;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use std::{fmt::Display, iter::Peekable};

#[derive(Debug, Clone)]
pub struct DefineMacro {
    pub label: String,
    pub value: String,
    pub value_kind: TokenType,
}

impl DefineMacro {
    pub fn new(label: &str, value: &str, kind: TokenType) -> Self {
        DefineMacro {
            label: label.to_string(),
            value: value.to_string(),
            value_kind: kind,
        }
    }

    pub fn replace_defines(tokens: &mut Vec<Token>) {
        let mut iter = tokens.iter_mut().peekable();
        let mut define_list: Vec<DefineMacro> = Vec::new();

        while let Some(mut current) = iter.next() {
            if current.kind == TokenType::DefineKeyword {
                // we have a define and we need to add it
                let _ = Self::create_definition(&mut iter, &mut define_list, &current);
            } else if current.kind == TokenType::Label {
                let _ = Self::sub_label(&mut current, &define_list);
            } else if current.kind == TokenType::UnDefineKeyword {
                let _ = Self::remove_definition(&mut iter, &mut define_list, &current);
            } else if current.kind == TokenType::Expression {
                let _ = Self::sub_expression(&mut current, &define_list);
            }
        }
    }
    pub fn sub_expression(token: &mut Token, define_macros: &Vec<DefineMacro>) -> bool {
        let label_exists = define_macros.iter().find(|def| {
            let stripped_token = &token.token;

            return stripped_token.contains(&def.label);
        });

        if label_exists.is_none() {
            return true;
        }

        let label_def = label_exists.unwrap();

        let new_token = token.token.replace(&label_def.label, &label_def.value);

        token.token = new_token;

        return true;
    }
    pub fn sub_label(token: &mut Token, define_macros: &Vec<DefineMacro>) -> bool {
        let label_exists = define_macros.iter().find(|def| {
            let stripped_token = token
                .token
                .strip_prefix("[")
                .unwrap_or(&token.token)
                .strip_suffix("]")
                .unwrap_or(&token.token);

            return def.label == stripped_token;
        });

        if label_exists.is_none() {
            return false;
        }
        let definition = label_exists.unwrap();

        let new_value = if token.is_addr {
            format!("[{}]", &definition.value)
        } else {
            definition.value.clone()
        };

        token.token = new_value;
        token.kind = definition.value_kind.clone();

        return true;
    }
    fn remove_definition<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a mut Token>>,
        define_list: &mut Vec<DefineMacro>,
        inital_token: &Token,
    ) -> bool {
        let label = iter.peek();
        if label.is_none() {
            Logging::log_preprocessor_error_info(
                "expected @undefine label",
                &inital_token.token_info,
            );
            return false;
        }

        let label = label.unwrap();

        if label.kind != TokenType::Label {
            Logging::log_preprocessor_error_info("expected define label", &label.token_info);
            return false;
        }
        let label = iter.next().unwrap();
        let index = define_list.iter().position(|define| {
            return define.label == label.token;
        });
        if index.is_none() {
            Logging::log_preprocessor_error_info(
                "undefine label does not match a know definition",
                &label.token_info,
            );
            return false;
        }
        let index = index.unwrap();

        define_list.swap_remove(index);

        return true;
    }
    fn create_definition<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a mut Token>>,
        define_list: &mut Vec<DefineMacro>,
        inital_token: &Token,
    ) -> bool {
        let label = iter.peek();

        if label.is_none() {
            Logging::log_preprocessor_error_info(
                "expected @define label",
                &inital_token.token_info,
            );
            return false;
        }
        let label = label.unwrap();

        if label.kind != TokenType::DefineDefinitionLabel {
            Logging::log_preprocessor_error_info("expected define label", &label.token_info);
            return false;
        }
        let label = iter.next().unwrap();

        let value = iter.peek();

        if value.is_none() {
            Logging::log_preprocessor_error_info(
                "expected @define value",
                &inital_token.token_info,
            );
        }

        let value = iter.next().unwrap();

        let dup_names = define_list
            .iter()
            .filter(|def| {
                return def.label == label.token;
            })
            .count()
            != 0;

        if dup_names {
            Logging::log_preprocessor_error_info("duplicate definitions found", &label.token_info);
            return false;
        }

        let macro_def = DefineMacro::new(&label.token, &value.token, value.kind.clone());

        define_list.push(macro_def);

        return true;
    }
}
impl Display for DefineMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
