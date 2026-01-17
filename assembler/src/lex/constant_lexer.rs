use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct ConstantLexer {}

impl ConstantLexer {
    const CONSTANT_TYPES: [(&'static str, TokenType); 6] = [
        (r"^0x[0-9A-Fa-f]+$", TokenType::Hex),
        (r"^0b[10]+$", TokenType::Binary),
        (r"^[\-+0-9]+$", TokenType::Decimal),
        (r"^'.'$", TokenType::Character),
        (r#"^".*"$"#, TokenType::String),
        (r"^<.*>$", TokenType::STDImportPath),
    ];
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

        for (expr, token_type) in Self::CONSTANT_TYPES {
            if Lexer::regex_match(expr, Lexer::remove_square_brackets(token)) {
                return Some(token_type);
            }
        }

        return None;
    }
}
