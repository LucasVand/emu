use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct CommentLexer {}

impl CommentLexer {
    const COMMENT_EXPR: &'static str = r"^;.*$";
    pub fn parse(
        token: &str,
        next: char,
        _line_num: usize,
        _parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        let token = token.trim();
        if next == '\n' && Lexer::regex_match(Self::COMMENT_EXPR, token) {
            return Some(TokenType::Comment);
        }
        return None;
    }
}
