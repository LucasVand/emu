use crate::preprocessor::preprocessor_error::PreprocessorError;
use crate::preprocessor::{
    define_macro::DefineMacro, macro_expansion::macro_preprocessing::MacroPreprocessing,
};
use crate::utils::syntax_error::AssemblerError;
use crate::utils::token::Token;

pub struct Preprocessor {}

impl Preprocessor {
    pub fn preprocess_tokens(tokens: Vec<Token>) -> (Vec<Token>, Vec<Box<dyn AssemblerError>>) {
        let mut error_list: Vec<PreprocessorError> = Vec::new();

        let (define_replaced_tokens, mut errors) = DefineMacro::replace_defines(tokens);
        error_list.append(&mut errors);

        let (macro_expanded_tokens, mut errors) =
            MacroPreprocessing::expand_macros(define_replaced_tokens);

        error_list.append(&mut errors);

        return (
            macro_expanded_tokens,
            error_list
                .iter()
                .map(|err| {
                    return Box::<dyn AssemblerError>::from(err);
                })
                .collect(),
        );
    }
}
