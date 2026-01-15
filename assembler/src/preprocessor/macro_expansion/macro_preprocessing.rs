use crate::preprocessor::macro_expansion::macro_definition::MacroDefinition;
use crate::preprocessor::macro_expansion::macro_expansion::MacroExpansion;
use crate::preprocessor::preprocessor_error::PreprocessorError;
use crate::utils::token::Token;
use crate::utils::token::TokenType;

pub struct MacroPreprocessing {}

impl MacroPreprocessing {
    pub fn expand_macros(tokens: Vec<Token>) -> (Vec<Token>, Vec<PreprocessorError>) {
        // TODO: allow for labels in macros by prepending the macro name and the invocation number
        // so that all the labels are unique after expassion

        let mut error_list = Vec::new();
        let mut new_token_list = Vec::new();

        // this is the master macro list
        let (macro_list, tokens_without_macros, mut errors) =
            MacroDefinition::create_macro_list(tokens);
        error_list.append(&mut errors);

        let mut iter = tokens_without_macros.into_iter().peekable();

        while let Some(token) = iter.next() {
            // if macro then we need to expand
            if token.kind == TokenType::MacroMnemonic {
                let expansion = MacroExpansion::expand_macro(&mut iter, &token, &macro_list);
                match expansion {
                    Ok(mut expand) => new_token_list.append(&mut expand),
                    Err(err) => error_list.push(err),
                }
            } else {
                new_token_list.push(token);
            }
        }
        return (new_token_list, error_list);
    }
}
