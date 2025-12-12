use crate::lex::token::Token;
use crate::lex::token::TokenType;
use crate::utils::logging::Logging;
use regex::Regex;

pub struct LabelLexer {}

impl LabelLexer {
    pub const REGEX_EXPRESSION: &'static str = r"^[a-zA-Z._][a-zA-z0-9_.]*:$";

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

        let token = Token::new(remove_end, TokenType::LabelDefinition, line_num, line);

        parsed_tokens.push(token);
        return true;
    }
}
