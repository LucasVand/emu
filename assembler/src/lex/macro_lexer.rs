use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use regex::Regex;

pub struct MacroLexer {}

impl MacroLexer {
    const REGEX_EXPRESSION: &'static str = r"^@macro\s*$";
    const END_REGEX_EXPRESSION: &'static str = r"^@end\s*$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line)
            || Regex::new(Self::END_REGEX_EXPRESSION)
                .unwrap()
                .is_match(line);
    }

    pub fn parse_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexerError> {
        // checks if its a @macro or @end line
        if line != "@macro" && line != "@end" {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "none"),
                LexerErrorType::ExpectedMacroDefineKeyword,
            ));
        }
        // gets the token type for it
        let token_type = if line != "@macro" {
            TokenType::EndKeyword
        } else {
            TokenType::MacroKeyword
        };

        let info = TokenInfo::new(line, line, line_num, "macro_lexer");
        let token = Token::new(line.to_string(), token_type, info);

        return Ok(vec![token]);
    }
}
