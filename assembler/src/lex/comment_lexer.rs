use std::sync::LazyLock;

use regex::Regex;

use crate::utils::token::{Token, TokenType};

pub struct CommentLexer {}

const COMMENT_EXPR: &'static str = r"^;.*$";
impl CommentLexer {
    pub fn parse(
        token: &str,
        next: char,
        _line_num: usize,
        _parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        static COMMENT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(COMMENT_EXPR).unwrap());

        let token = token.trim();
        if next == '\n' && COMMENT_REGEX.is_match(token) {
            return Some(TokenType::Comment);
        }
        return None;
    }
}
