use std::sync::LazyLock;

use regex::Regex;

use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};
const EXPRESSION_EXPR: &'static str = r"^\([0-9A-Za-z_.+\-*/<>()\s%$]+\)$";

pub struct ExpressionLexer {}

impl ExpressionLexer {
    pub fn parse(
        token: &str,
        next: char,
        _line_num: usize,
        _parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        static EXPRESSION_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(EXPRESSION_EXPR).unwrap());

        let token = Lexer::remove_square_brackets(token.trim());
        if !EXPRESSION_REGEX.is_match(token) {
            return None;
        }

        if !Lexer::check_brackets('(', ')', token) {
            return None;
        }
        if Lexer::SEPERATOR_CHARS.contains(next) {
            return Some(TokenType::Expression);
        }
        return None;
    }
}
