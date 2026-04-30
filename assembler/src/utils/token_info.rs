#[derive(PartialEq, Debug, Clone)]
pub struct TokenInfo {
    pub line_num: usize,
    pub line: String,
    pub token: String,
    pub lexer_classification: String,
    pub is_address: bool,
    pub file_location: String,
}

impl TokenInfo {
    pub fn new(
        line: &str,
        token: &str,
        line_num: usize,
        lexer_class: &str,
        is_address: bool,
        file_path: impl AsRef<std::path::Path>,
    ) -> TokenInfo {
        let absolute_path = std::fs::canonicalize(file_path.as_ref())
            .unwrap_or_else(|_| file_path.as_ref().to_path_buf())
            .to_string_lossy()
            .to_string();

        TokenInfo {
            is_address: is_address,
            line_num: line_num,
            line: line.to_string(),
            token: token.to_string(),
            lexer_classification: lexer_class.to_string(),
            file_location: absolute_path,
        }
    }
}
