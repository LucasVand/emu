use crate::lex::constant_lexer::ConstantLexer;
use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use regex::Regex;

pub struct DefineLexer {}

impl DefineLexer {
    const DEFINE_EXPRESSION: &'static str = r"^@define .*$";
    const UNDEFINE_EXPRESSION: &'static str = r"^@undefine .*$";
    const LABEL_EXPRESSION: &'static str = r"[a-zA-Z_.][a-zA-Z0-9_.]*";

    fn check_regex(expr: &str, haystack: &str) -> bool {
        return Regex::new(expr).unwrap().is_match(haystack);
    }
    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::DEFINE_EXPRESSION).unwrap().is_match(line)
            || Regex::new(Self::UNDEFINE_EXPRESSION)
                .unwrap()
                .is_match(line);
    }
    pub fn parse_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexerError> {
        if Regex::new(Self::DEFINE_EXPRESSION).unwrap().is_match(line) {
            return Self::parse_define_line(line, line_num);
        } else {
            return Self::parse_undefine_line(line, line_num);
        }
    }
    fn parse_undefine_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexerError> {
        let mut split = line.splitn(2, " ");
        let keyword = split.next();
        let label = split.next();

        if keyword.is_none() || Some("@undefine") != keyword {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "undefine_lexer"),
                LexerErrorType::ExpectedUndefineKeyword,
            ));
        }

        if label.is_none() {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "undefine_lexer"),
                LexerErrorType::ExpectedUndefineLabel,
            ));
        }

        let keyword = keyword.unwrap().trim();
        let label = label.unwrap().trim();

        let info_keyword = TokenInfo::new(line, keyword, line_num, "define_lexer");
        let keyword_token = Token::new(
            keyword.to_string(),
            TokenType::UnDefineKeyword,
            info_keyword,
        );
        let info_label = TokenInfo::new(line, label, line_num, "define_lexer");
        let label_token = Token::new(label.to_string(), TokenType::Label, info_label.clone());

        if !Self::check_regex(Self::LABEL_EXPRESSION, label) {
            return Err(LexerError::new(
                info_label,
                LexerErrorType::InvalidUndefineLabel,
            ));
        }

        return Ok(vec![keyword_token, label_token]);
    }

    fn parse_define_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexerError> {
        let mut split = line.splitn(3, " ");
        let keyword = split.next();
        let label = split.next();
        let value = split.next();

        if keyword.is_none() || keyword != Some("@define") {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "define_lexer"),
                LexerErrorType::ExpectedDefineKeyword,
            ));
        }

        if label.is_none() {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "define_lexer"),
                LexerErrorType::ExpectedDefineLabel,
            ));
        }

        if value.is_none() {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "define_lexer"),
                LexerErrorType::ExpectedDefineValue,
            ));
        }

        let keyword = keyword.unwrap();
        let value = value.unwrap();
        let label = label.unwrap().trim();

        let info_keyword = TokenInfo::new(line, keyword, line_num, "define_lexer");
        let keyword_token = Token::new(keyword.to_string(), TokenType::DefineKeyword, info_keyword);
        let info_label = TokenInfo::new(line, label, line_num, "define_lexer");
        let label_token = Token::new(
            label.to_string(),
            TokenType::DefineDefinitionLabel,
            info_label.clone(),
        );

        if !Self::check_regex(Self::LABEL_EXPRESSION, label) {
            return Err(LexerError::new(
                info_label,
                LexerErrorType::InvalidDefineLabel,
            ));
        }

        let value_token = ConstantLexer::parse_instruction_operand(value, line, line_num)?;

        return Ok(vec![keyword_token, label_token, value_token]);
    }
}
