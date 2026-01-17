use crate::preprocessor::preprocessor_error::PreprocessorError;
use crate::preprocessor::preprocessor_error::PreprocessorErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use std::fmt::Display;
use std::vec;

#[derive(Debug, Clone)]
pub struct MacroDefinition {
    pub label: String,
    pub parameters: Vec<Token>,
    pub value: Vec<Token>,
    pub info: TokenInfo,
}
impl PartialEq for MacroDefinition {
    fn eq(&self, other: &Self) -> bool {
        if !(self.label == other.label) {
            return false;
        }
        let mut params1 = self.parameters.iter();
        let mut params2 = other.parameters.iter();
        if params1.len() != params2.len() {
            return false;
        }

        for _ in 0..params1.len() {
            let token1 = params1.next().unwrap();
            let token2 = params2.next().unwrap();

            if token1.token != token2.token {
                return false;
            }
        }

        return true;
    }
}
impl Display for MacroDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl MacroDefinition {
    pub fn new(label: &str, info: TokenInfo) -> Self {
        MacroDefinition {
            label: label.to_string(),
            parameters: Vec::new(),
            value: Vec::new(),
            info: info,
        }
    }

    pub fn create_macro_list(
        token_list: Vec<Token>,
    ) -> (Vec<MacroDefinition>, Vec<Token>, Vec<PreprocessorError>) {
        let mut iter = token_list.into_iter();
        let mut error_list = Vec::new();
        let mut macro_list: Vec<MacroDefinition> = Vec::new();
        let mut new_token_list: Vec<Token> = Vec::new();

        while let Some(ele) = iter.next() {
            if ele.kind == TokenType::MacroKeyword {
                let macro_def = Self::create_macro_definition(ele, &mut iter);
                match macro_def {
                    Ok(def) => {
                        if macro_list.contains(&def) {
                            error_list.push(PreprocessorError::new(
                                def.info.clone(),
                                PreprocessorErrorType::DuplicateDefinitionsFound,
                            ));
                        } else {
                            macro_list.push(def);
                        }
                    }
                    Err(err) => error_list.push(err),
                }
            } else {
                new_token_list.push(ele);
            }
        }

        return (macro_list, new_token_list, error_list);
    }
    pub fn create_macro_definition(
        token: Token,
        iter: &mut vec::IntoIter<Token>,
    ) -> Result<MacroDefinition, PreprocessorError> {
        let macro_mnemonic = iter.next();

        if macro_mnemonic.is_none() {
            return Err(PreprocessorError::new(
                token.token_info.clone(),
                PreprocessorErrorType::ExpectedMacroDefinitinMnemonic,
            ));
        }

        let macro_def = macro_mnemonic.unwrap();

        if macro_def.kind != TokenType::MacroDefinitionMnemonic {
            return Err(PreprocessorError::new(
                token.token_info.clone(),
                PreprocessorErrorType::ExpectedMacroDefinitinMnemonic,
            ));
        }

        let mut macro_obj = MacroDefinition::new(&macro_def.token, macro_def.token_info.clone());
        loop {
            let current = iter.next();
            if current.is_none() {
                return Err(PreprocessorError::new(
                    token.token_info.clone(),
                    PreprocessorErrorType::MissingMacroEndKeyword,
                ));
            }
            let current = current.unwrap();
            if current.kind == TokenType::MacroKeyword {
                return Err(PreprocessorError::new(
                    current.token_info,
                    PreprocessorErrorType::NestedMacros,
                ));
            } else if current.kind == TokenType::MacroDefinitionParameter {
                macro_obj.parameters.push(current.clone());
            } else if current.kind == TokenType::EndKeyword {
                return Ok(macro_obj);
            } else {
                macro_obj.value.push(current.clone());
            }
        }
    }
}
