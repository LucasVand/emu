use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Token {
    pub token: String,
    pub is_addr: bool,
    pub kind: TokenType,
    pub line_num: usize,
    pub line: String,
}
impl Token {
    pub fn new(token: &str, kind: TokenType, line_num: usize, line: &str) -> Token {
        Token {
            is_addr: false,
            token: token.to_string(),
            kind,
            line_num: line_num,
            line: line.to_string(),
        }
    }
    pub fn new_address(token: &str, kind: TokenType, line_num: usize, line: &str) -> Token {
        Token {
            is_addr: true,
            token: token.to_string(),
            kind,
            line_num: line_num,
            line: line.to_string(),
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

    // Data Defining Types
    MacroParameter,
    Expression,
    String,
    Hex,
    Binary,
    Character,
    Decimal,
}
impl TokenType {
    pub const INSTRUCTION_OPERANDS: [TokenType; 8] = [
        TokenType::Label,
        TokenType::Expression,
        TokenType::String,
        TokenType::Hex,
        TokenType::Binary,
        TokenType::Character,
        TokenType::Decimal,
        TokenType::Register,
    ];
    pub const LITERALS: [TokenType; 7] = [
        TokenType::Label,
        TokenType::Expression,
        TokenType::String,
        TokenType::Hex,
        TokenType::Binary,
        TokenType::Character,
        TokenType::Decimal,
    ];
    pub const DATA_DEFINITIONS: [TokenType; 3] = [
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
