use std::{error::Error, fmt::Display};

use crate::utils::{
    syntax_error::{AssemblerError, AssemblerStage},
    token_info::TokenInfo,
};

#[derive(Debug, Clone)]
pub enum PreprocessorErrorType {
    // define errors
    ExpectedDefineLabel,
    ExpectedDefineValue,
    DuplicateDefinitionsFound,
    UnableToFindDefinition,
    ExpectedUndefineLabel,
    ExpectedUndefineValue,
    UnableToFindDefinitionToRemove,

    // macro errors
    ExpectedMacroDefinitinMnemonic,
    DuplicateMacroDefinitions,
    MissingMacroEndKeyword,
    UnableToFindMacroDefinition,
    UnableToFindMacroParameter,
    InfiniteRecursionMacro,
}

impl Error for PreprocessorErrorType {}

impl Display for PreprocessorErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ExpectedDefineLabel => "Expected @define label",
            Self::ExpectedDefineValue => "Expected @define value",
            Self::DuplicateDefinitionsFound => "Duplicate definitions found",
            Self::UnableToFindDefinition => "Unable to find definition",
            Self::ExpectedUndefineLabel => "Expected @undefine label",
            Self::ExpectedUndefineValue => "Expected @undefine value",
            Self::UnableToFindDefinitionToRemove => "Unable to find @define to ,remove",
            Self::ExpectedMacroDefinitinMnemonic => "Expected macro definition",
            Self::DuplicateMacroDefinitions => "Found duplicate macro definitions",
            Self::MissingMacroEndKeyword => "Missing end keyword in macro",
            Self::UnableToFindMacroDefinition => "Unable to find macro definition",
            Self::UnableToFindMacroParameter => "Unable to find macro parameter",
            Self::InfiniteRecursionMacro => "Cannot call a macro from in itself",
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct PreprocessorError {
    info: TokenInfo,
    error: PreprocessorErrorType,
}
impl PreprocessorError {
    pub fn new(info: TokenInfo, err: PreprocessorErrorType) -> PreprocessorError {
        PreprocessorError {
            info: info,
            error: err,
        }
    }
}
impl AssemblerError for PreprocessorError {
    fn stage(&self) -> AssemblerStage {
        return AssemblerStage::Preprocessor;
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
impl From<&PreprocessorError> for Box<dyn AssemblerError> {
    fn from(value: &PreprocessorError) -> Self {
        return Box::new(value.clone());
    }
}
