use std::{error::Error, fmt::Display, fs};

use common::levenshtein_distance::LevenshteinDistance;

use crate::utils::{
    syntax_error::{AssemblerError, AssemblerStage},
    token_info::TokenInfo,
};

#[derive(Debug, Clone)]
pub enum IncludeErrorType {
    NoUserImportFound,
    NoSTDImportFound { path_to_std: String },
    DuplicateImports,
    UnableToOpenFile,
}
#[derive(Debug, Clone)]
pub struct IncludeError {
    info: TokenInfo,
    error: IncludeErrorType,
}
impl IncludeError {
    pub fn new(info: TokenInfo, err: IncludeErrorType) -> IncludeError {
        IncludeError {
            info: info,
            error: err,
        }
    }
}
impl From<&IncludeError> for Box<dyn AssemblerError> {
    fn from(value: &IncludeError) -> Box<dyn AssemblerError> {
        return Box::new(value.clone());
    }
}

impl Error for IncludeErrorType {}

impl Display for IncludeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            IncludeErrorType::NoUserImportFound => "Could not resolve import",
            IncludeErrorType::NoSTDImportFound { path_to_std: _ } => "Could not resolve std import",
            IncludeErrorType::DuplicateImports => "Duplicate imports found",
            IncludeErrorType::UnableToOpenFile => "Unable to open file",
        };

        write!(f, "{}", message)
    }
}

impl AssemblerError for IncludeError {
    fn fix(&self) -> Option<String> {
        match &self.error {
            IncludeErrorType::NoSTDImportFound { path_to_std } => {
                let files = Self::dir_files(path_to_std);
                return match files {
                    Ok(dir) => {
                        // finds the std that is the closest in spelling
                        let mut lowest: (f32, String) = (1000.0, "".to_string());
                        // look over all the std entries
                        for file_name in dir {
                            let dist = LevenshteinDistance::distance(&file_name, &lowest.1);
                            if dist < lowest.0 {
                                lowest = (dist, file_name.to_string());
                            }
                        }

                        let final_str = lowest.1.to_string();
                        return Some(format!("Did you mean {}?", final_str));
                    }
                    Err(_err) => Some("Unable to read files from std".to_string()),
                };
            }
            _ => None,
        }
    }
    fn error(&self) -> Box<dyn Error> {
        return Box::new(self.error.clone());
    }
    fn info(&self) -> &TokenInfo {
        return &self.info;
    }
    fn stage(&self) -> AssemblerStage {
        return AssemblerStage::Imports;
    }
}
impl IncludeError {
    fn dir_files(path: &str) -> Result<Vec<String>, std::io::Error> {
        let files = fs::read_dir(format!("./{}", path))?;
        let mut list: Vec<String> = Vec::new();
        for file in files {
            if let Ok(file) = file {
                if let Some(name) = file.file_name().to_str() {
                    list.push(name.to_string());
                }
            }
        }

        return Ok(list);
    }
}
