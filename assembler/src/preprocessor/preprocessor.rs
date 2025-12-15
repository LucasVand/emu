use crate::{
    lex::token::Token,
    preprocessor::{
        define_macro::DefineMacro, macro_expansion::macro_preprocessing::MacroPreprocessing,
    },
};

pub struct Preprocessor {}

impl Preprocessor {
    pub fn preprocess_tokens(tokens: &Vec<Token>) -> Vec<Token> {
        let mut tokens_clone = tokens.clone();
        // TODO: allow zero operand macros

        DefineMacro::replace_defines(&mut tokens_clone);

        MacroPreprocessing::expand_macros(&mut tokens_clone);

        return tokens_clone;
    }
}
