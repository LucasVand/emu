use regex::Regex;

use crate::{
    lex::token::{Token, TokenType},
    utils::logging::Logging,
};

pub struct DefineLexer {}

impl DefineLexer {
    const REGEX_EXPRESSION: &'static str = r"^@define [a-zA-z0-9]+ [a-zA-z0-9]+$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }

    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let mut split = line.split_whitespace();
        let keyword = split.nth(0);
        let label = split.nth(0);
        let value = split.nth(0);

        if keyword.is_none() || keyword != Some("@define") {
            Logging::log_lexer_error("expected @define", line_num, line);
            return false;
        }

        if label.is_none() {
            Logging::log_lexer_error("expected label", line_num, line);
            return false;
        }

        if value.is_none() {
            Logging::log_lexer_error("expected value", line_num, line);
            return false;
        }

        let keyword = keyword.unwrap();
        let value = value.unwrap();
        let label = label.unwrap();

        let keyword_token = Token::new(keyword, TokenType::DefineKeyword, line_num);
        let value = Token::new(value, TokenType::DefineValue, line_num);
        let label = Token::new(label, TokenType::DefineLabel, line_num);

        parsed_tokens.push(keyword_token);
        parsed_tokens.push(label);
        parsed_tokens.push(value);
        return true;
    }
}
