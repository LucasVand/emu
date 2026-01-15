use std::fmt::Display;

use crate::utils::token_info::TokenInfo;

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub token: String,
    pub is_addr: bool,
    pub kind: TokenType,
    pub token_info: TokenInfo,
}

impl Token {
    pub fn new(token: String, kind: TokenType, info: TokenInfo) -> Token {
        Token {
            is_addr: false,
            token: token.to_string(),
            kind,
            token_info: info,
        }
    }
    pub fn new_address(token: String, kind: TokenType, info: TokenInfo) -> Token {
        Token {
            is_addr: true,
            token: token.to_string(),
            kind,
            token_info: info,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Mnemonic,
    MacroMnemonic,
    Register,
    Label,
    LabelDefinition,

    DefineDefinitionLabel,
    DefineKeyword,
    UnDefineKeyword,

    MacroDefinitionParameter,
    MacroKeyword,
    MacroDefinitionMnemonic,
    EndKeyword,

    // Data Definition
    WordDataDefineKeyword,
    DoubleWordDataDefineKeyword,
    StringDataDefineKeyword,
    SpaceDataDefineKeyword,

    // Data Defining Types
    MacroParameter,
    Expression,
    String,
    Hex,
    Binary,
    Character,
    Decimal,

    // Double Register
    DoubleRegister,
}
impl TokenType {
    pub const INSTRUCTION_OPERANDS: [TokenType; 9] = [
        TokenType::Label,
        TokenType::Expression,
        TokenType::String,
        TokenType::Hex,
        TokenType::Binary,
        TokenType::Character,
        TokenType::Decimal,
        TokenType::Register,
        TokenType::DoubleRegister,
    ];
    pub const LITERALS: [TokenType; 8] = [
        TokenType::Label,
        TokenType::Expression,
        TokenType::String,
        TokenType::Hex,
        TokenType::Binary,
        TokenType::Character,
        TokenType::Decimal,
        TokenType::DoubleRegister,
    ];
    pub const DATA_DEFINITIONS: [TokenType; 4] = [
        TokenType::SpaceDataDefineKeyword,
        TokenType::WordDataDefineKeyword,
        TokenType::DoubleWordDataDefineKeyword,
        TokenType::StringDataDefineKeyword,
    ];
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
