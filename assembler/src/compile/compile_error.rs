use std::{error::Error, fmt::Display};

use crate::{
    compile::operand::Operand,
    utils::{
        syntax_error::{AssemblerError, AssemblerStage},
        token_info::TokenInfo,
    },
};

#[derive(Debug, Clone)]
pub enum CompilerErrorType {
    UnableToFindLabel,
    DuplicateLabelsFound,
    NoMainEntryPointFound,

    //instruction compiler
    IncorrectNumberOfOperands { found: usize, expected: usize },
    IncorrectOperandFound { found: Operand, expected: Operand },

    // data compiler
    CannotParseIntoWord,
    CannotParseIntoDoubleWord,
    CharacterIsNotValidASCII,
    ExpectedString,
    ExpectedDataDefinition,
}

impl Error for CompilerErrorType {}

impl Display for CompilerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::UnableToFindLabel => "Unable to find label",
            Self::DuplicateLabelsFound => "Duplicate labels found",
            Self::NoMainEntryPointFound => "No main entry point found",
            Self::IncorrectNumberOfOperands { found, expected } => &format!(
                "Incorrect number of operands found, expected {} but found {}",
                expected, found
            ),
            Self::IncorrectOperandFound { found, expected } => &format!(
                "Incorrect operand found, expected ({}) but found ({})",
                expected, found
            ),
            Self::CannotParseIntoWord => "This cannot be compiled into a word",
            Self::CannotParseIntoDoubleWord => "This cannot be compiled into a double word ",
            Self::CharacterIsNotValidASCII => "This character is not valid ascii",
            Self::ExpectedString => "Expected string",
            Self::ExpectedDataDefinition => "Expected data definition",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    info: TokenInfo,
    error: CompilerErrorType,
}
impl CompilerError {
    pub fn new(info: TokenInfo, err: CompilerErrorType) -> CompilerError {
        CompilerError {
            info: info,
            error: err,
        }
    }
}
impl AssemblerError for CompilerError {
    fn stage(&self) -> AssemblerStage {
        return AssemblerStage::Compiler;
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
impl From<&CompilerError> for Box<dyn AssemblerError> {
    fn from(value: &CompilerError) -> Self {
        return Box::new(value.clone());
    }
}
