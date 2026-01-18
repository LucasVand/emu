use regex::Regex;
use std::sync::LazyLock;

use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct RegisterLexer {}

const REGISTER_LIST: [&'static str; 8] = ["a", "b", "c", "d", "l", "h", "z", "f"];
const DOUBLE_REGISTER: &'static str = r"^[abcdhlfz]{2}$";

impl RegisterLexer {
    pub fn parse(
        token: &str,
        next: char,
        _line_num: usize,
        _parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        static DOUBLE_REGISTER_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(DOUBLE_REGISTER).unwrap());

        let token = token.trim();
        if !Lexer::SEPERATOR_CHARS.contains(next) {
            return None;
        }
        if REGISTER_LIST.contains(&token) {
            return Some(TokenType::Register);
        }

        if DOUBLE_REGISTER_REGEX.is_match(Lexer::remove_square_brackets(token)) {
            return Some(TokenType::DoubleRegister);
        }

        return None;
    }
}
