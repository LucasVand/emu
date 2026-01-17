use std::{fs, io::ErrorKind};

use regex::Regex;

use crate::{
    includes::include_error::{IncludeError, IncludeErrorType},
    utils::{syntax_error::AssemblerError, token_info::TokenInfo},
};

pub struct Includes {}

impl Includes {
    const IMPORT_EXPRESSION: &'static str = r#"^@include\s*["<].*[">]\s*$"#;
    const STD_EXPRESSION: &'static str = r"<.*>";
    // resolves all imports and returns a new string along with all the errors that occurred
    pub fn resolve_imports(
        contents: String,
        path_to_std: &str,
    ) -> (String, Vec<Box<dyn AssemblerError>>) {
        // import paths that have already been resolved
        let mut resolved_imports: Vec<String> = Vec::new();
        // vec of errors
        let mut errors: Vec<IncludeError> = Vec::new();
        // the new contents
        let mut new_contents = contents.clone();
        // do we have at least one import
        let lines = contents.split("\n");

        for (index, line) in lines.enumerate() {
            // if the line is an import
            if Regex::new(Self::IMPORT_EXPRESSION).unwrap().is_match(&line) {
                // get the import
                let resolved_import = Self::resolve_import(line, index, path_to_std);
                // if its an error add to errors
                match resolved_import {
                    Ok((import_value, path)) => {
                        // if the import has already been resolved
                        if resolved_imports.contains(&path) {
                            let info = TokenInfo::new(line, &path, index, "Import", false);

                            errors
                                .push(IncludeError::new(info, IncludeErrorType::DuplicateImports));
                        } else {
                            new_contents = new_contents.replacen(line, &import_value, 1);
                            resolved_imports.push(path);
                        }
                    }
                    Err(err) => {
                        // remove the line
                        new_contents = new_contents.replacen(line, "", 1);
                        errors.push(err)
                    }
                }
            }
        }

        let mapped_err = errors
            .iter()
            .map(|err| return Box::<dyn AssemblerError>::from(err))
            .collect();

        return (new_contents, mapped_err);
    }

    // finds the value of an import
    fn resolve_import(
        line: &str,
        line_num: usize,
        path_to_std: &str,
    ) -> Result<(String, String), IncludeError> {
        // split the line
        let mut parts = line.splitn(2, " ");
        parts.next();
        // this is the import path
        let path = parts.next().unwrap().trim();

        // is this a std import
        let is_std = Regex::new(Self::STD_EXPRESSION).unwrap().is_match(path);

        // create the debugging token info
        let info = TokenInfo::new(line, path, line_num, "Include", false);

        if is_std {
            let trimmed_path = path
                .strip_prefix("<")
                .unwrap_or(path)
                .strip_suffix(">")
                .unwrap_or(path);

            return Ok((
                Self::std_import(trimmed_path, path_to_std, info)?,
                trimmed_path.to_string(),
            ));
        } else {
            let trimmed_path = path
                .strip_prefix("\"")
                .unwrap_or(path)
                .strip_suffix("\"")
                .unwrap_or(path);

            return Ok((
                Self::custom_import(trimmed_path, info)?,
                trimmed_path.to_string(),
            ));
        }
    }

    fn std_import(path: &str, path_to_std: &str, info: TokenInfo) -> Result<String, IncludeError> {
        let file = fs::read_to_string(format!("{}/{}", path_to_std, path));

        if let Err(err) = file {
            return Err(IncludeError::new(
                info,
                match err.kind() {
                    ErrorKind::NotFound => IncludeErrorType::NoSTDImportFound {
                        path_to_std: path_to_std.to_string(),
                    },
                    _ => IncludeErrorType::UnableToOpenFile,
                },
            ));
        }
        return Ok(file.unwrap());
    }
    fn custom_import(path: &str, info: TokenInfo) -> Result<String, IncludeError> {
        let file = fs::read_to_string(format!("./asm/{}", path));

        if let Err(err) = file {
            return Err(IncludeError::new(
                info,
                match err.kind() {
                    ErrorKind::NotFound => IncludeErrorType::NoUserImportFound,
                    _ => IncludeErrorType::UnableToOpenFile,
                },
            ));
        }
        return Ok(file.unwrap());
    }
}
