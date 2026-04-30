use std::{error::Error, fmt::Display};

use crate::utils::{
    syntax_error::{AssemblerError, AssemblerStage},
    token_info::TokenInfo,
};

#[derive(Debug, Clone)]
pub enum LexerErrorType {
    TokenDoesNotMatch,
}

impl Error for LexerErrorType {}

impl Display for LexerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::TokenDoesNotMatch => "Token does not match",
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct LexerError {
    info: TokenInfo,
    error: LexerErrorType,
}
impl LexerError {
    pub fn new(info: TokenInfo, err: LexerErrorType) -> LexerError {
        LexerError {
            info: info,
            error: err,
        }
    }
}
impl AssemblerError for LexerError {
    fn stage(&self) -> AssemblerStage {
        return AssemblerStage::Lexer;
    }
    fn fix(&self) -> Option<String> {
        return None;
    }
    fn info(&self) -> &TokenInfo {
        return &self.info;
    }
    fn error(&self) -> Box<dyn Error> {
        return Box::new(self.error.clone());
    }
}
impl From<&LexerError> for Box<dyn AssemblerError> {
    fn from(value: &LexerError) -> Self {
        return Box::new(value.clone());
    }
}
