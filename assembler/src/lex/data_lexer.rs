use regex::Regex;

use crate::{
    lex::{
        constant_lexer::ConstantLexer,
        token::{Token, TokenType},
    },
    utils::logging::Logging,
};

pub struct DataLexer {}

impl DataLexer {
    const REGEX_EXPRESSION: &'static str = r"^@d[bds] .+$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }
    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let str_sections = line.to_string();
        let mut sections = str_sections.splitn(2, " ");

        let first_section = sections.next();
        let second_section = sections.next();

        if first_section.is_none() {
            Logging::log_lexer_error("expected data type defintion", line_num, line);
            return false;
        }
        if second_section.is_none() {
            Logging::log_lexer_error("expected data definition", line_num, line);
            return false;
        }

        let first_section = first_section.unwrap();
        let second_section = second_section.unwrap();

        let data_char: char = first_section.trim().chars().nth(2).unwrap();
        let first_token_type = Self::token_type_from_char(data_char);
        let data_keyword_token =
            Token::new(first_section, first_token_type.clone(), line_num, line);

        parsed_tokens.push(data_keyword_token);

        let data = second_section.split(",");

        for ele in data {
            ConstantLexer::parse_constant_data(ele, line, parsed_tokens, line_num);
        }
        return true;
    }
    pub fn token_type_from_char(letter: char) -> TokenType {
        match letter {
            's' => TokenType::StringDataDefineKeyword,
            'b' => TokenType::WordDataDefineKeyword,
            'd' => TokenType::DoubleWordDataDefineKeyword,
            _ => panic!("Invalid data type"),
        }
    }
}
