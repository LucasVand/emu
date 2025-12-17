use std::fmt::Display;

use crate::utils::token_info::TokenInfo;
#[derive(Debug, Clone)]
pub enum CompiledToken {
    Binary { byte: u8, info: TokenInfo },
    Label { name: String, info: TokenInfo },
}
impl CompiledToken {
    pub fn create_label(label: &str, info: &TokenInfo) -> CompiledToken {
        CompiledToken::Label {
            name: label.to_string(),
            info: info.clone(),
        }
    }
    pub fn create_token(value: u8, info: &TokenInfo) -> CompiledToken {
        CompiledToken::Binary {
            byte: value,
            info: info.clone(),
        }
    }
    pub fn create_tokens(value: u16, info: &TokenInfo) -> [CompiledToken; 2] {
        [
            CompiledToken::Binary {
                byte: (value >> 8) as u8,
                info: info.clone(),
            },
            CompiledToken::Binary {
                byte: value as u8,
                info: info.clone(),
            },
        ]
    }
}

impl Display for CompiledToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
