use regex::Regex;

use crate::{
    lex::token::{Token, TokenType},
    utils::logging::Logging,
};

pub struct MacroLexer {}

impl MacroLexer {
    const REGEX_EXPRESSION: &'static str = r"^@macro$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }

    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        if line != "@macro" {
            Logging::log_lexer_error("expected @marco", line_num, line);
            return false;
        }

        let token = Token::new(line, TokenType::MacroKeyword, line_num);
        parsed_tokens.push(token);

        return true;
    }
}
