use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct ExpressionLexer {}

impl ExpressionLexer {
    const EXPRESSION_EXPR: &'static str = r"^\([0-9A-Za-z_.+\-*/<>()\s%$]+\)$";
    pub fn parse(
        token: &str,
        next: char,
        _line_num: usize,
        _parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        let token = Lexer::remove_square_brackets(token.trim());
        if !Lexer::regex_match(Self::EXPRESSION_EXPR, token) {
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
