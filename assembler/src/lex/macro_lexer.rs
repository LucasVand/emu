use crate::lex::token::Token;
use crate::lex::token::TokenType;
use crate::utils::logging::Logging;
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

    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        // checks if its a @macro or @end line
        if line != "@macro" && line != "@end" {
            Logging::log_lexer_error("expected @marco", line_num, line);
            return false;
        }
        // gets the token type for it
        let token_type = if line != "@macro" {
            TokenType::EndKeyword
        } else {
            TokenType::MacroKeyword
        };

        let token = Token::new(line, token_type, line_num, line);
        parsed_tokens.push(token);

        return true;
    }
}
