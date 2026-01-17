#[derive(PartialEq, Debug, Clone)]
pub struct TokenInfo {
    pub line_num: usize,
    pub line: String,
    pub token: String,
    pub lexer_classification: String,
    pub is_address: bool,
}

impl TokenInfo {
    pub fn new(
        line: &str,
        token: &str,
        line_num: usize,
        lexer_class: &str,
        is_address: bool,
    ) -> TokenInfo {
        TokenInfo {
            is_address: is_address,
            line_num: line_num,
            line: line.to_string(),
            token: token.to_string(),
            lexer_classification: lexer_class.to_string(),
        }
    }
}
