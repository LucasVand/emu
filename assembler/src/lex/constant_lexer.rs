use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use regex::Regex;

pub struct ConstantLexer {}

impl ConstantLexer {
    const REGISTER_LIST: [&'static str; 8] = ["a", "b", "c", "d", "l", "h", "z", "f"];

    const MACRO_PARAMETER_REGEX: &'static str = r"^(%[rix]\d+|\[%[rix]\d+\])$";
    const CHARACTER_REGEX: &'static str = r"^'[\x00-\x7F]'$";
    const STRING_REGEX: &'static str = "^\"([\x00-\x7F])*\"$";
    const HEX_REGEX: &'static str = r"^[-+]?0x[0-9abcdefABCDEF]+$";
    const BIN_REGEX: &'static str = r"^[-+]?0b[10]+$";
    const DEC_REGEX: &'static str = r"^[-+]?[0-9]+$";
    const EXPRESSION_REGEX: &'static str = r"^\([$0-9A-Za-z_. \+\-\*/<>\(\)%ri]+\)$";
    const LABEL: &'static str = r"^[a-zA-Z0-9._]+$";
    const ADDRESS: &'static str = r"^\[.+\]$";
    const DOUBLE_REGISTER_ADDRESS: &'static str = r"^[abcdlhzf][abcdhlzf]$";

    pub fn parse_constant_data(
        constant: &str,
        line: &str,
        line_num: usize,
    ) -> Result<Token, LexerError> {
        return Self::parse_constant(constant, line, line_num, true);
    }
    pub fn parse_instruction_operand(
        constant: &str,
        line: &str,
        line_num: usize,
    ) -> Result<Token, LexerError> {
        return Self::parse_constant(constant, line, line_num, false);
    }

    fn parse_constant(
        constant: &str,
        line: &str,
        line_num: usize,
        is_data: bool,
    ) -> Result<Token, LexerError> {
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
            return Err(LexerError::new(info, LexerErrorType::UnableToParseConstant));
        }

        // we use trimmed here because we want to keep the [] around the constant
        let token = if is_addr {
            Token::new_address(trimmed.to_string(), token_type, info)
        } else {
            Token::new(trimmed.to_string(), token_type, info)
        };

        return Ok(token);
    }
    fn check_expression(exp: &str, value: &str) -> bool {
        return Regex::new(exp).unwrap().is_match(value);
    }
}
