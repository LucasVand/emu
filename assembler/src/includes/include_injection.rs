use std::collections::HashSet;
use std::fs;
use std::io::ErrorKind;

use crate::{
    includes::include_error::{IncludeError, IncludeErrorType},
    lex::lexer::Lexer,
    utils::{syntax_error::AssemblerError, token::Token, token::TokenType, token_info::TokenInfo},
};

pub struct IncludeInjection {}

impl IncludeInjection {
    /// Injects resolved includes into a token stream.
    /// Takes a vector of tokens and resolves all @include directives,
    /// replacing them with the lexed tokens from the imported files.
    /// Prevents duplicate imports using a HashSet of resolved paths.
    pub fn inject_includes(
        tokens: Vec<Token>,
        path_to_std: &str,
    ) -> (Vec<Token>, Vec<Box<dyn AssemblerError>>) {
        let mut resolved_imports = HashSet::new();
        let mut errors: Vec<Box<dyn AssemblerError>> = Vec::new();

        let (result, mut new_errors) = Self::inject_includes_internal(
            tokens.into_iter(),
            path_to_std,
            &mut resolved_imports,
        );

        errors.append(&mut new_errors);

        (result, errors)
    }

    /// Internal recursive function for injecting includes.
    /// Maintains the set of already-resolved imports to prevent duplicates.
    fn inject_includes_internal(
        mut tokens: impl Iterator<Item = Token>,
        path_to_std: &str,
        resolved_imports: &mut HashSet<String>,
    ) -> (Vec<Token>, Vec<Box<dyn AssemblerError>>) {
        let mut result: Vec<Token> = Vec::new();
        let mut errors: Vec<Box<dyn AssemblerError>> = Vec::new();

        while let Some(token) = tokens.next() {
            // Check if this is an @include keyword
            if token.kind == TokenType::IncludeKeyword {
                // Next token should be the import path
                if let Some(path_token) = tokens.next() {
                    let import_path = &path_token.token;
                    
                    // Determine if it's a std import using the is_std pattern
                    let is_std = import_path.starts_with('<') && import_path.ends_with('>');
                    
                    let (file_content_result, import_key) = if is_std {
                        // Standard library import
                        let trimmed = import_path
                            .strip_prefix("<").unwrap_or("")
                            .strip_suffix(">").unwrap_or("");
                        (Self::load_std_import(trimmed, path_to_std, &path_token.token_info), trimmed.to_string())
                    } else if import_path.starts_with('"') && import_path.ends_with('"') {
                        // User import
                        let trimmed = import_path
                            .strip_prefix('"').unwrap_or("")
                            .strip_suffix('"').unwrap_or("");
                        (Self::load_custom_import(trimmed, &path_token.token_info), trimmed.to_string())
                    } else {
                        // Invalid import format, skip
                        result.push(token);
                        continue;
                    };

                    // Check if already resolved to prevent duplicates
                    if resolved_imports.contains(&import_key) {
                        // Skip duplicate import
                        continue;
                    }

                    // Mark as resolved
                    resolved_imports.insert(import_key);

                    // Try to load the file content
                    match file_content_result {
                        Ok((content, file_path)) => {
                            // Lex the imported content with the file path
                            let (imported_tokens, mut lex_errors) = Lexer::parse_str(content, file_path);

                            // Add any lexer errors
                            errors.append(&mut lex_errors);

                            // Recursively resolve imports in the imported tokens
                            let (injected_tokens, mut injection_errors) = 
                                Self::inject_includes_internal(
                                    imported_tokens.into_iter(),
                                    path_to_std,
                                    resolved_imports,
                                );

                            // Add any errors from the recursive injection
                            errors.append(&mut injection_errors);

                            // Inject the resolved tokens
                            result.extend(injected_tokens);
                        }
                        Err(err) => {
                            // Add error
                            errors.push(err);
                        }
                    }
                } else {
                    // No path token following @include, just add the token
                    result.push(token);
                }
            } else {
                // Regular token, just add it
                result.push(token);
            }
        }

        (result, errors)
    }

    /// Load a standard library import file
    fn load_std_import(
        path: &str,
        path_to_std: &str,
        token_info: &TokenInfo,
    ) -> Result<(String, String), Box<dyn AssemblerError>> {
        let file_path = format!("{}/{}", path_to_std, path);
        match fs::read_to_string(&file_path) {
            Ok(content) => Ok((content, file_path)),
            Err(err) => {
                let error_type = match err.kind() {
                    ErrorKind::NotFound => IncludeErrorType::NoSTDImportFound {
                        path_to_std: path_to_std.to_string(),
                    },
                    _ => IncludeErrorType::UnableToOpenFile,
                };
                Err(Box::new(IncludeError::new(token_info.clone(), error_type)))
            }
        }
    }

    /// Load a custom user import file
    fn load_custom_import(
        path: &str,
        token_info: &TokenInfo,
    ) -> Result<(String, String), Box<dyn AssemblerError>> {
        let file_path = format!("./asm/{}", path);
        match fs::read_to_string(&file_path) {
            Ok(content) => Ok((content, file_path)),
            Err(err) => {
                let error_type = match err.kind() {
                    ErrorKind::NotFound => IncludeErrorType::NoUserImportFound,
                    _ => IncludeErrorType::UnableToOpenFile,
                };
                Err(Box::new(IncludeError::new(token_info.clone(), error_type)))
            }
        }
    }
}
