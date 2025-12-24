use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use regex::Regex;

use crate::utils::logging::Logging;

pub struct ConstantLexer {}

impl ConstantLexer {
    const REGISTER_LIST: [&'static str; 8] = ["a", "b", "c", "d", "l", "h", "z", "f"];

    const MACRO_PARAMETER_REGEX: &'static str = r"^(%[ri]\d+|\[%[ri]\d+\])$";
    const CHARACTER_REGEX: &'static str = r"^'[\x00-\x7F]'$";
    const STRING_REGEX: &'static str = "^\"([\x00-\x7F])*\"$";
    const HEX_REGEX: &'static str = r"^[-+]?0x[0-9abcdefABCDEF]+$";
    const BIN_REGEX: &'static str = r"^[-+]?0b[10]+$";
    const DEC_REGEX: &'static str = r"^[-+]?[0-9]+$";
    const EXPRESSION_REGEX: &'static str = r"^\([$0-9A-Za-z_. \+\-\*/<>\(\)]+\)$";
    const LABEL: &'static str = r"^[a-zA-Z0-9._]+$";
    const ADDRESS: &'static str = r"^\[.+\]$";
    const DOUBLE_REGISTER_ADDRESS: &'static str = r"^[abcdlhzf][abcdhlzf]$";

    pub fn parse_constant_data(
        constant: &str,
        line: &str,
        parsed_tokens: &mut Vec<Token>,
        line_num: usize,
    ) -> bool {
        return Self::parse_constant(constant, line, parsed_tokens, line_num, true);
    }
    pub fn parse_instruction_operand(
        constant: &str,
        line: &str,
        parsed_tokens: &mut Vec<Token>,
        line_num: usize,
    ) -> bool {
        return Self::parse_constant(constant, line, parsed_tokens, line_num, false);
    }

    fn parse_constant(
        constant: &str,
        line: &str,
        parsed_tokens: &mut Vec<Token>,
        line_num: usize,
        is_data: bool,
    ) -> bool {
        let trimmed = constant.trim();

        let is_addr = Self::check_expression(Self::ADDRESS, trimmed);

        // constant without []
        let constant_trimmed = trimmed
            .strip_suffix("]")
            .unwrap_or(trimmed)
            .strip_prefix("[")
            .unwrap_or(trimmed);

        let info = TokenInfo::new(line, trimmed, line_num, "constant_lexer");

        let token_type: TokenType;
        if Self::check_expression(Self::CHARACTER_REGEX, constant_trimmed) {
            token_type = TokenType::Character;
        } else if is_data && Self::check_expression(Self::STRING_REGEX, constant_trimmed) {
            token_type = TokenType::String;
        } else if Self::check_expression(Self::HEX_REGEX, constant_trimmed) {
            token_type = TokenType::Hex;
        } else if Self::check_expression(Self::BIN_REGEX, constant_trimmed) {
            token_type = TokenType::Binary;
        } else if Self::check_expression(Self::DEC_REGEX, constant_trimmed) {
            token_type = TokenType::Decimal;
        } else if Self::check_expression(Self::EXPRESSION_REGEX, constant_trimmed) {
            token_type = TokenType::Expression;
        } else if !is_data && Self::check_expression(Self::MACRO_PARAMETER_REGEX, constant_trimmed)
        {
            token_type = TokenType::MacroParameter;
        } else if !is_data && Self::REGISTER_LIST.contains(&constant_trimmed) {
            token_type = TokenType::Register;
        } else if !is_data
            && Self::check_expression(Self::DOUBLE_REGISTER_ADDRESS, constant_trimmed)
        {
            token_type = TokenType::DoubleRegister
        } else if Self::check_expression(Self::LABEL, constant_trimmed) {
            token_type = TokenType::Label;
        } else {
            Logging::log_lexer_error_info("unable to parse constant", &info);
            return false;
        }

        // we use trimmed here because we want to keep the [] around the constant
        let token = if is_addr {
            Token::new_address(trimmed, token_type, info)
        } else {
            Token::new(trimmed, token_type, info)
        };

        parsed_tokens.push(token);

        return true;
    }
    fn check_expression(exp: &str, value: &str) -> bool {
        return Regex::new(exp).unwrap().is_match(value);
    }
}
