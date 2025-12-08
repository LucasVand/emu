use std::fmt::Display;

pub struct Token {
    pub token: String,
    pub kind: TokenType,
    pub line: usize,
}
impl Token {
    pub fn new(token: &str, kind: TokenType, line: usize) -> Token {
        Token {
            token: token.to_string(),
            kind,
            line,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Token: {}, Kind: {}, Line: {})",
            self.token, self.kind, self.line
        )
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Mnemonic,
    Register,
    Label,
    MacroIdenifier,

    DefineLabel,
    DefineKeyword,
    DefineValue,

    MacroKeyword,

    // Data Definition
    WordDataDefineKeyword,
    DoubleWordDataDefineKeyword,
    StringDataDefineKeyword,

    // Data Defining Types
    Address,
    Expression,
    Contant,
    Hex,
    Binary,
    Character,
    Decimal,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
