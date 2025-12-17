#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub line_num: usize,
    pub line: String,
    pub token: String,
    pub lexer_classification: String,
}

impl TokenInfo {
    pub fn new(line: &str, token: &str, line_num: usize, lexer_class: &str) -> TokenInfo {
        TokenInfo {
            line_num: line_num,
            line: line.to_string(),
            token: token.to_string(),
            lexer_classification: lexer_class.to_string(),
        }
    }
}
