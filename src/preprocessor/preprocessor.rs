use crate::{
    lex::token::{Token, TokenType},
    preprocessor::{define_macro::DefineMacro, macro_expansion::Macro},
    utils::logging::Logging,
};

pub struct Preprocessor {}

impl Preprocessor {
    pub fn preprocess_tokens(tokens: &Vec<Token>) -> Vec<Token> {
        let mut tokens_clone = tokens.clone();

        DefineMacro::replace_defines(&mut tokens_clone);

        Macro::expand_macros(&mut tokens_clone);

        return tokens_clone;
    }
}
