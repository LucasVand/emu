use regex::Regex;

use crate::{
    lex::token::{Token, TokenType},
    utils::logging::Logging,
};

pub struct LabelLexer {}

impl LabelLexer {
    const REGEX_EXPRESSION: &'static str = r"^[a-zA-z0-9_.]+:$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }

    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let remove_end = line.strip_suffix(":");

        if remove_end.is_none() {
            Logging::log_lexer_error("expected label", line_num, line);
            return false;
        }
        let remove_end = remove_end.unwrap();

        let token = Token::new(remove_end, TokenType::Label, line_num);

        parsed_tokens.push(token);
        return true;
    }
}
