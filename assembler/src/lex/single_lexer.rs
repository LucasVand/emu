use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct SingleLexer {}

impl SingleLexer {
    pub fn parse(
        token: &str,
        _next: char,
        _line_num: usize,
        _parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        if token.len() != 1 {
            return None;
        }
        if Lexer::SEPERATOR_CHARS.contains(token) && token != ";" && token != "" {
            return Some(TokenType::SingleChar);
        }

        return None;
    }
}
