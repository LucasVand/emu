use crate::lex::constant_lexer::ConstantLexer;
use crate::lex::token::Token;
use crate::lex::token::TokenType;
use crate::utils::logging::Logging;
use regex::Regex;

pub struct DefineLexer {}

impl DefineLexer {
    const REGEX_EXPRESSION: &'static str = r"^@define [a-zA-z0-9_.]+ .+$";
    const UNDEFINE_EXPRESSION: &'static str = r"^@undefine [a-zA-Z0-9._]+\s*$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line)
            || Regex::new(Self::UNDEFINE_EXPRESSION)
                .unwrap()
                .is_match(line);
    }
    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        if Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line) {
            return Self::parse_define_line(line, parsed_tokens, line_num);
        } else {
            return Self::parse_undefine_line(line, parsed_tokens, line_num);
        }
    }
    fn parse_undefine_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let mut split = line.splitn(2, " ");
        let keyword = split.next();
        let label = split.next();

        if keyword.is_none() || Some("@undefine") != keyword {
            Logging::log_lexer_error("expected @undefine keyword", line_num, line);
            return false;
        }

        if label.is_none() {
            Logging::log_lexer_error("expected @undefine label", line_num, line);
            return false;
        }

        let keyword = keyword.unwrap();
        let label = label.unwrap();

        let token = Token::new(keyword, TokenType::UnDefineKeyword, line_num, line);
        let label_token = Token::new(label, TokenType::Label, line_num, line);

        parsed_tokens.push(token);
        parsed_tokens.push(label_token);

        return true;
    }

    fn parse_define_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let mut split = line.splitn(3, " ");
        let keyword = split.next();
        let label = split.next();
        let value = split.next();

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

        let keyword_token = Token::new(keyword, TokenType::DefineKeyword, line_num, line);
        let label = Token::new(label, TokenType::DefineDefinitionLabel, line_num, line);

        parsed_tokens.push(keyword_token);
        parsed_tokens.push(label);

        let ret = ConstantLexer::parse_instruction_operand(value, line, parsed_tokens, line_num);
        if !ret {
            return false;
        }

        return true;
    }
}
