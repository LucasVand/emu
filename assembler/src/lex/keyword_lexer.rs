use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct KeywordLexer {}

impl KeywordLexer {
    const KEYWORDS: [(&'static str, TokenType); 9] = [
        ("@define", TokenType::DefineKeyword),
        ("@macro", TokenType::MacroKeyword),
        ("@end", TokenType::EndKeyword),
        ("@include", TokenType::IncludeKeyword),
        ("@undefine", TokenType::UnDefineKeyword),
        ("@db", TokenType::WordDataDefineKeyword),
        ("@dd", TokenType::DoubleWordDataDefineKeyword),
        ("@dw", TokenType::SpaceDataDefineKeyword),
        ("@ds", TokenType::StringDataDefineKeyword),
    ];
    pub fn parse(
        token: &str,
        next: char,
        _line_num: usize,
        _parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        let token = token.trim();
        // it is not a seperator then we don't care
        if !Lexer::SEPERATOR_CHARS.contains(next) {
            return None;
        }

        for (keyword, token_type) in Self::KEYWORDS {
            if token == keyword {
                return Some(token_type);
            }
        }

        return None;
    }
}
