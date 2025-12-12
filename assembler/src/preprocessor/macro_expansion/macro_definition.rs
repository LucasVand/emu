use crate::lex::token::TokenType;
use crate::utils::logging::Logging;
use std::fmt::Display;

use crate::lex::token::Token;

#[derive(Debug)]
pub struct MacroDefinition {
    pub label: String,
    pub parameters: Vec<Token>,
    pub value: Vec<Token>,
}
impl Display for MacroDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl MacroDefinition {
    pub fn new(label: &str) -> Self {
        MacroDefinition {
            label: label.to_string(),
            parameters: Vec::new(),
            value: Vec::new(),
        }
    }

    pub fn create_macro_list(token_list: &Vec<Token>) -> Vec<MacroDefinition> {
        let mut iter = token_list.iter();
        let mut macro_list: Vec<MacroDefinition> = Vec::new();

        while let Some(ele) = iter.next() {
            if ele.kind == TokenType::MacroKeyword {
                Self::parse_macro(ele, &mut iter, &mut macro_list);
            }
        }

        return macro_list;
    }
    pub fn parse_macro<'a>(
        token: &Token,
        iter: &mut impl Iterator<Item = &'a Token>,
        macro_list: &mut Vec<MacroDefinition>,
    ) -> bool {
        let macro_def = iter.next();

        if macro_def.is_none() {
            Logging::log_preprocessor_error(
                "expected macro definition",
                token.line_num,
                &token.line,
            );
            return false;
        }

        let macro_def = macro_def.unwrap();

        if macro_def.kind != TokenType::MacroDefinitionMnemonic {
            Logging::log_preprocessor_error_specific(
                "expected macro definition",
                macro_def.line_num,
                &macro_def.line,
                &macro_def.token,
            );
            return false;
        }

        let mut macro_obj = MacroDefinition::new(&macro_def.token);
        loop {
            let current = iter.next();
            if current.is_none() {
                Logging::log_preprocessor_error(
                    "missing the end of a macro definition",
                    token.line_num,
                    &token.line,
                );
                return false;
            }
            let current = current.unwrap();

            if current.kind == TokenType::MacroDefinitionParameter {
                macro_obj.parameters.push(current.clone());
            } else if current.kind == TokenType::EndKeyword {
                macro_list.push(macro_obj);
                return true;
            } else {
                macro_obj.value.push(current.clone());
            }
        }
    }
}
