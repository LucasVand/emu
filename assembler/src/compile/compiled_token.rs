use std::fmt::Display;
#[derive(Debug, PartialEq, Clone)]
pub enum CompiledToken {
    Binary { byte: u8 },
    Label { name: String },
}
impl CompiledToken {
    pub fn create_label(label: &str) -> CompiledToken {
        CompiledToken::Label {
            name: label.to_string(),
        }
    }
    pub fn create_token(value: u8) -> CompiledToken {
        CompiledToken::Binary { byte: value }
    }
    pub fn create_tokens(value: u16) -> [CompiledToken; 2] {
        [
            CompiledToken::Binary {
                byte: (value >> 8) as u8,
            },
            CompiledToken::Binary { byte: value as u8 },
        ]
    }
}

impl Display for CompiledToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
