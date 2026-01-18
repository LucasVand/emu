use std::{error::Error, fmt::Display};

use common::levenshtein_distance::LevenshteinDistance;

use crate::{
    preprocessor::macro_expansion::{
        macro_definition::MacroDefinition, macro_parameters::TypedMacroParameter,
    },
    utils::{
        syntax_error::{AssemblerError, AssemblerStage},
        token_info::TokenInfo,
    },
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
    UnableToFindMacroDefinition {
        closest: Vec<String>,
    },
    UnableToFindMacroParameter,
    InfiniteRecursionMacro,
    IncorrectNumberOfOperands {
        expected: usize,
        found: usize,
    },
    IncorrectMacroOperands {
        expected: TypedMacroParameter,
        found: TypedMacroParameter,
    },
    NestedMacros,
}
impl PreprocessorErrorType {
    pub fn new_unable_to_find_macro_definition(
        search: &str,
        def_list: &Vec<MacroDefinition>,
    ) -> PreprocessorErrorType {
        let mut closest_list = Vec::new();
        let mut smallest = 1000.0;
        for def in def_list {
            let dist = LevenshteinDistance::distance_no_case(&def.label, search);
            if dist == smallest && !closest_list.contains(&def.label) {
                closest_list.push(def.label.to_string());
            } else if dist < smallest {
                smallest = dist;
                closest_list.clear();
                closest_list.push(def.label.to_string());
            }
        }

        return PreprocessorErrorType::UnableToFindMacroDefinition {
            closest: closest_list,
        };
    }
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
            Self::UnableToFindMacroDefinition { closest: _ } => "Unable to find macro definition",
            Self::UnableToFindMacroParameter => "Unable to find macro parameter",
            Self::InfiniteRecursionMacro => "Cannot call a macro from in itself",
            Self::NestedMacros => "Cannot have nested macros",
            Self::IncorrectNumberOfOperands { found, expected } => &format!(
                "Incorrect number of operands found, expected {} but found {}",
                expected, found
            ),
            Self::IncorrectMacroOperands { found, expected } => &format!(
                "Incorrect operand found, expected {} but found {}",
                expected, found
            ),
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
        match &self.error {
            PreprocessorErrorType::UnableToFindMacroDefinition { closest } => {
                if closest.is_empty() {
                    return None;
                }

                let mut res = String::new();
                res.push_str("Did you mean ("); // add the inital message

                for name in closest {
                    res.push_str(&format!("{},", name)); // add all the names of closest macros
                }
                res.pop(); // remove the last comma
                res.push_str(")"); // push the closing brace
                return Some(res);
            }
            _ => None,
        }
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
