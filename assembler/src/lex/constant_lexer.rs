use regex::Regex;
use std::sync::LazyLock;

use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct ConstantLexer {}

impl ConstantLexer {
    pub fn parse(
        token: &str,
        next: char,
        _line_num: usize,
        _parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        let token = token.trim();
        if !Lexer::SEPERATOR_CHARS.contains(next) {
            return None;
        }
        static CONSTANT_TYPES: [(LazyLock<Regex>, TokenType); 6] = [
            (
                LazyLock::new(|| Regex::new(r"^0x[0-9A-Fa-f]+$").unwrap()),
                TokenType::Hex,
            ),
            (
                LazyLock::new(|| Regex::new(r"^0b[10]+$").unwrap()),
                TokenType::Binary,
            ),
            (
                LazyLock::new(|| Regex::new(r"^[\-+0-9]+$").unwrap()),
                TokenType::Decimal,
            ),
            (
                LazyLock::new(|| Regex::new(r"^'.'$").unwrap()),
                TokenType::Character,
            ),
            (
                LazyLock::new(|| Regex::new(r#"^".*"$"#).unwrap()),
                TokenType::String,
            ),
            (
                LazyLock::new(|| Regex::new(r"^<.*>$").unwrap()),
                TokenType::STDImportPath,
            ),
        ];

        for (expr, token_type) in CONSTANT_TYPES.iter() {
            if expr.is_match(Lexer::remove_square_brackets(token)) {
                return Some(token_type.clone());
            }
        }

        return None;
    }
}
