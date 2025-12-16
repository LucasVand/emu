use crate::lex::token::Token;
use crate::lex::token::TokenType;

pub struct ParseLiteral {}

impl ParseLiteral {
    pub fn parse_u8(token: &Token) -> u8 {
        let base64 = Self::parse_literal(token);
        return base64 as u8;
    }
    pub fn parse_u16(token: &Token) -> u16 {
        let base64 = Self::parse_literal(token);
        return base64 as u16;
    }
    fn parse_literal(token: &Token) -> u64 {
        let token_token = &token.token;

        let token_brackets_stripped = token_token
            .strip_prefix("[")
            .unwrap_or(token_token)
            .strip_suffix("]")
            .unwrap_or(token_token);

        let token_brackets_plus_stripped = token_brackets_stripped
            .strip_prefix("+")
            .unwrap_or(token_brackets_stripped);

        let is_neg = token_brackets_stripped.strip_prefix("-").is_some();

        let token_str = token_brackets_plus_stripped
            .strip_prefix("-")
            .unwrap_or(token_brackets_plus_stripped);

        let value: Option<u64>;
        if token.kind == TokenType::Decimal {
            let res = token_str.parse::<u64>();
            value = res.ok();
        } else if token.kind == TokenType::Character {
            let char = token_str
                .strip_prefix("'")
                .unwrap()
                .strip_suffix("'")
                .unwrap()
                .chars()
                .next()
                .unwrap();

            value = Some(char as u64);
        } else if token.kind == TokenType::Hex {
            let hex = token_str.strip_prefix("0x").unwrap_or(token_str);
            let base64 = u64::from_str_radix(hex, 16).expect("this should be valid");

            value = Some(base64 as u64);
        } else if token.kind == TokenType::Binary {
            let bin = token_str.strip_prefix("0b").unwrap_or(token_str);
            let base64 = u64::from_str_radix(bin, 2).expect("this should be valid");

            value = Some(base64);
        } else {
            value = None;
        }

        if value.is_none() {
            panic!("could not parse literal: {}", token_str);
        }
        let value = value.unwrap();
        if is_neg {
            return !value + 1;
        }

        return value;
    }
}
