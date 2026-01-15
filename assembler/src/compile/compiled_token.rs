use core::panic;
use std::fmt::Display;

use crate::utils::token_info::TokenInfo;
#[derive(Debug, Clone)]
pub enum CompiledToken {
    Binary {
        byte: u8,
        info: TokenInfo,
    },
    DoubleWord {
        byte1: u8,
        byte2: u8,
        info: TokenInfo,
    },
    Label {
        name: String,
        info: TokenInfo,
    },
    Expression {
        expr: String,
        double_word: bool,
        info: TokenInfo,
    },
}
impl CompiledToken {
    pub fn create_expression(
        expression: String,
        double_word: bool,
        info: TokenInfo,
    ) -> CompiledToken {
        CompiledToken::Expression {
            expr: expression,
            info: info,
            double_word: double_word,
        }
    }
    pub fn create_label(label: String, info: TokenInfo) -> CompiledToken {
        CompiledToken::Label {
            name: label,
            info: info,
        }
    }
    pub fn create_word(value: u8, info: TokenInfo) -> CompiledToken {
        CompiledToken::Binary {
            byte: value,
            info: info,
        }
    }
    pub fn create_double_word(value: u16, info: TokenInfo) -> CompiledToken {
        CompiledToken::DoubleWord {
            byte1: (value >> 8) as u8,
            byte2: (value as u8),
            info: info,
        }
    }
    pub fn compile_btyes(&self, bytes: &mut Vec<u8>) {
        match self {
            CompiledToken::Label { name: _, info: _ } => {
                panic!("Should not find label")
            }
            CompiledToken::Expression {
                expr: _,
                double_word: _,
                info: _,
            } => {
                panic!("Should not find expression")
            }
            CompiledToken::Binary { byte, info: _ } => {
                bytes.push(*byte);
            }
            CompiledToken::DoubleWord {
                byte1,
                byte2,
                info: _,
            } => {
                bytes.push(*byte1);
                bytes.push(*byte2);
            }
        }
    }
    pub fn size(&self) -> usize {
        match self {
            CompiledToken::Label { name: _, info: _ } => {
                return 2;
            }
            CompiledToken::Binary { byte: _, info: _ } => {
                return 1;
            }
            CompiledToken::DoubleWord {
                byte1: _,
                byte2: _,
                info: _,
            } => {
                return 2;
            }
            CompiledToken::Expression {
                expr: _,
                double_word,
                info: _,
            } => {
                let size = if *double_word { 2 } else { 1 };
                return size;
            }
        };
    }
}

impl Display for CompiledToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
