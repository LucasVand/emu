use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct RegisterLexer {}

impl RegisterLexer {
    const REGISTER_LIST: [&'static str; 8] = ["a", "b", "c", "d", "l", "h", "z", "f"];
    const DOUBLE_REGISTER: &'static str = r"^[abcdhlfz]{2}$";

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
        if Self::REGISTER_LIST.contains(&token) {
            return Some(TokenType::Register);
        }

        if Lexer::regex_match(Self::DOUBLE_REGISTER, Lexer::remove_square_brackets(token)) {
            return Some(TokenType::DoubleRegister);
        }

        return None;
    }
}
