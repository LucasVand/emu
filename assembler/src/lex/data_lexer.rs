use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use regex::Regex;

use crate::lex::constant_lexer::ConstantLexer;

pub struct DataLexer {}

impl DataLexer {
    const REGEX_EXPRESSION: &'static str = r"^@d[bdsw] .+$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }
    pub fn parse_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexerError> {
        let mut token_list = Vec::new();
        let str_sections = line.to_string();
        let mut sections = str_sections.splitn(2, " ");

        let first_section = sections.next();
        let second_section = sections.next();

        if first_section.is_none() {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "data_lexer"),
                LexerErrorType::ExpectedDataTypeDefinition,
            ));
        }
        if second_section.is_none() {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "data_lexer"),
                LexerErrorType::ExpectedDataDefinition,
            ));
        }

        let first_section = first_section.unwrap();
        let second_section = second_section.unwrap();

        let data_char: char = first_section.trim().chars().nth(2).unwrap();
        let first_token_type = Self::token_type_from_char(data_char);

        let data_keyword_info = TokenInfo::new(line, first_section, line_num, "data_lexer");
        let data_keyword_token = Token::new(
            first_section.to_string(),
            first_token_type.clone(),
            data_keyword_info,
        );

        token_list.push(data_keyword_token);

        let data = second_section.split(",");

        for ele in data {
            let data_token = ConstantLexer::parse_constant_data(ele, line, line_num)?;
            token_list.push(data_token);
        }
        return Ok(token_list);
    }
    pub fn token_type_from_char(letter: char) -> TokenType {
        match letter {
            's' => TokenType::StringDataDefineKeyword,
            'b' => TokenType::WordDataDefineKeyword,
            'd' => TokenType::DoubleWordDataDefineKeyword,
            'w' => TokenType::SpaceDataDefineKeyword,
            _ => panic!("Invalid data type"),
        }
    }
}
