use std::{fmt::Display, iter::Peekable};

use crate::utils::{logging::Logging, token_info::TokenInfo};

#[derive(Debug)]
pub struct ExpressionToken {
    pub info: TokenInfo,
    pub kind: ExpressionTokenType,
}
impl Display for ExpressionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(PartialEq, Debug)]
pub enum ExpressionTokenType {
    Decimal { num: isize },
    Plus,
    Minus,
    Divide,
    Multiply,
    LBrack,
    RBrack,
    LeftShift,
    RightShift,
    Not,
}

impl ExpressionToken {
    const SINGLE_CHAR_TOKEN: &'static str = "-+/*()!";
    const DIGIT_TOKEN: &'static str = "xb1234567890";
    const SHIFT_CHAR_TOKEN: &'static str = "<>";

    pub fn left_binding_power(&self) -> usize {
        match self.kind {
            ExpressionTokenType::Decimal { num: _ } => 0,
            ExpressionTokenType::Plus => 10,
            ExpressionTokenType::Minus => 10,
            ExpressionTokenType::Divide => 20,
            ExpressionTokenType::LBrack => 0,
            ExpressionTokenType::RBrack => 0,
            ExpressionTokenType::Multiply => 20,
            ExpressionTokenType::Not => 40,
            ExpressionTokenType::LeftShift => 10,
            ExpressionTokenType::RightShift => 10,
        }
    }

    pub fn tokenize_expression(expr: &str, info: &TokenInfo) -> Vec<ExpressionToken> {
        let mut tokens: Vec<ExpressionToken> = Vec::new();
        let mut chs = expr.chars().peekable();

        while let Some(ch) = chs.next() {
            if Self::SINGLE_CHAR_TOKEN.contains(ch) {
                Self::parse_single_char_token(&mut tokens, ch, info);
            } else if Self::SHIFT_CHAR_TOKEN.contains(ch) {
                Self::parse_shift_token(&mut chs, &mut tokens, ch, info);
            } else if Self::DIGIT_TOKEN.contains(ch) {
                Self::parse_number_token(&mut chs, &mut tokens, ch, info);
            } else if ch == '\'' {
                Self::parse_character_token(&mut chs, &mut tokens, ch, info);
            } else if !ch.is_whitespace() {
                let mut new_info = info.clone();
                new_info.token = ch.to_string();
                Logging::log_compiler_error_info("found unexpected token", &new_info);
            }
        }

        return tokens;
    }
    fn parse_shift_token(
        chs: &mut Peekable<impl Iterator<Item = char>>,
        tokens: &mut Vec<ExpressionToken>,
        ch: char,
        info: &TokenInfo,
    ) {
        let mut new_info = info.clone();
        let mut token = String::new();
        token.push(ch);
        while let Some(new_ch) = chs.peek() {
            if *new_ch != ch {
                break;
            } else {
                token.push(chs.next().unwrap());
            }
        }
        new_info.token = token.clone();

        match token.as_str() {
            //TODO: add the shift op
            "<<" => {
                let tok = ExpressionToken {
                    info: new_info,
                    kind: ExpressionTokenType::LeftShift,
                };
                tokens.push(tok);
            }
            ">>" => {
                let tok = ExpressionToken {
                    info: new_info,
                    kind: ExpressionTokenType::RightShift,
                };
                tokens.push(tok);
            }
            _ => {
                Logging::log_compiler_error_info("expected shift operator", &new_info);
            }
        }
    }
    fn parse_character_token(
        chs: &mut Peekable<impl Iterator<Item = char>>,
        tokens: &mut Vec<ExpressionToken>,
        _ch: char,
        info: &TokenInfo,
    ) {
        let mut new_info = info.clone();

        let mut token = String::new();
        while let Some(ch) = chs.next() {
            if ch == '\'' {
                break;
            }
            token.push(ch);
        }
        new_info.token = format!("\'{}\'", token);

        if token.is_empty() {
            Logging::log_compiler_error_info("cannot have empty character", &new_info);
            return;
        }

        if token.len() != 1 {
            Logging::log_compiler_error_info("more then one character found", &new_info);
            return;
        }

        let ch = token.chars().next().unwrap();

        let dec = ch as isize;

        let expr = ExpressionToken {
            info: new_info,
            kind: ExpressionTokenType::Decimal { num: dec },
        };
        tokens.push(expr);
    }
    fn parse_number_token(
        chs: &mut Peekable<impl Iterator<Item = char>>,
        tokens: &mut Vec<ExpressionToken>,
        ch: char,
        info: &TokenInfo,
    ) {
        let mut new_info = info.clone();
        let mut token: String = String::new();
        token.push(ch);
        while let Some(ch) = chs.peek() {
            if Self::DIGIT_TOKEN.contains(*ch) {
                token.push(chs.next().unwrap());
            } else {
                break;
            }
        }
        new_info.token = token.clone();

        let num = Self::parse_number(&token);
        if num.is_none() {
            Logging::log_compiler_error_info("unable to parse literal", &new_info);
            return;
        }
        let exp = ExpressionToken {
            info: new_info,
            kind: ExpressionTokenType::Decimal {
                num: num.unwrap() as isize,
            },
        };

        tokens.push(exp);
    }
    fn parse_single_char_token(tokens: &mut Vec<ExpressionToken>, ch: char, info: &TokenInfo) {
        let kind = match ch {
            '+' => ExpressionTokenType::Plus,
            '-' => ExpressionTokenType::Minus,
            '/' => ExpressionTokenType::Divide,
            '*' => ExpressionTokenType::Multiply,
            '(' => ExpressionTokenType::LBrack,
            ')' => ExpressionTokenType::RBrack,
            _ => panic!("found in valid char when parsing single char token"),
        };
        let token = ExpressionToken {
            info: info.clone(),
            kind: kind,
        };
        tokens.push(token);
    }
    fn parse_number(token: &str) -> Option<usize> {
        if let Some(bin) = token.strip_prefix("0b") {
            let num = usize::from_str_radix(bin, 2);
            return num.ok();
        } else if let Some(hex) = token.strip_prefix("0x") {
            let num = usize::from_str_radix(hex, 16);
            return num.ok();
        } else {
            let num = usize::from_str_radix(token, 10);
            return num.ok();
        }
    }
}
