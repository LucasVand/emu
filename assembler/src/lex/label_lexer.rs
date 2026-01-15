use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use regex::Regex;

pub struct LabelLexer {}

impl LabelLexer {
    pub const REGEX_EXPRESSION: &'static str = r"^[a-zA-Z._][a-zA-z0-9_.]*:$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }

    pub fn parse_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexerError> {
        let remove_end = line.strip_suffix(":");

        if remove_end.is_none() {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "label_lexer"),
                LexerErrorType::ExpectedLabelDefinition,
            ));
        }
        let remove_end = remove_end.unwrap();

        let info = TokenInfo::new(line, remove_end, line_num, "label_lexer");
        let token = Token::new(remove_end, TokenType::LabelDefinition, info);

        return Ok(vec![token]);
    }
}
