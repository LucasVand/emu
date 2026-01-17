use std::{error::Error, fmt::Display};

use crate::utils::{
    syntax_error::{AssemblerError, AssemblerStage},
    token_info::TokenInfo,
};

#[derive(Debug, Clone)]
pub enum LexerErrorType {
    TokenDoesNotMatch,
    LineDoesNotMatch,
    ExpectedMacroDefineKeyword,
    ExpectedMacroLabel,
    InvalidMacroDefinitionParameter,
    ExpectedLabelDefinition,
    ExpectedInstructionMnemonic,
    UnableToParseConstant,
    ExpectedUndefineKeyword,
    ExpectedUndefineLabel,
    ExpectedDefineLabel,
    ExpectedDefineKeyword,
    ExpectedDefineValue,
    ExpectedDataTypeDefinition,
    ExpectedDataDefinition,
    InvalidDefineLabel,
    InvalidUndefineLabel,
}

impl Error for LexerErrorType {}

impl Display for LexerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::TokenDoesNotMatch => "Token does not match",
            Self::LineDoesNotMatch => "Line does not match any known expression",
            Self::ExpectedMacroDefineKeyword => "Expected macro define keyword",
            Self::ExpectedMacroLabel => "Expected macro label",
            Self::InvalidMacroDefinitionParameter => "Invalid macro definition parameter",
            Self::ExpectedLabelDefinition => "Expected label definition",
            Self::UnableToParseConstant => "Unable to parse constant",
            Self::ExpectedInstructionMnemonic => "Expected instruction mnemonic",
            Self::ExpectedUndefineKeyword => "Expected undefine keyword",
            Self::ExpectedUndefineLabel => "Expected undefine label",
            Self::ExpectedDefineKeyword => "Expected define keyword",
            Self::ExpectedDefineLabel => "Expected define label",
            Self::ExpectedDefineValue => "Expected define value",
            Self::ExpectedDataTypeDefinition => "Expected data type definition",
            Self::ExpectedDataDefinition => "Expected data definition",
            Self::InvalidDefineLabel => "Invalid @define label can only be [a-z][A-Z][0-9][_.]",
            Self::InvalidUndefineLabel => "Invalid @define label can only be [a-z][A-Z][0-9][_.]",
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
