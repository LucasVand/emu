use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;

pub struct EntryPoint {}

impl EntryPoint {
    /// Creates the injected entry point tokens: lda [main]\njnz 1\n@include <always.asm>
    /// These are injected at the beginning of every assembly file to set up the runtime.
    /// Using tokens instead of text preserves accurate line numbers in error messages.
    pub fn create_entry_point_tokens(filename: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        // Line 0: "lda [main]"
        tokens.push(Token::new(
            "lda".to_string(),
            TokenType::Mnemonic,
            TokenInfo::new("lda [main]", "lda", 0, "entry_point", false, filename),
        ));

        tokens.push(Token::new_address(
            "[main]".to_string(),
            TokenType::Label,
            TokenInfo::new("lda [main]", "[main]", 0, "entry_point", true, filename),
        ));

        // Line 1: "jnz 1"
        tokens.push(Token::new(
            "jnz".to_string(),
            TokenType::Mnemonic,
            TokenInfo::new("jnz 1", "jnz", 1, "entry_point", false, filename),
        ));

        tokens.push(Token::new(
            "1".to_string(),
            TokenType::Decimal,
            TokenInfo::new("jnz 1", "1", 1, "entry_point", false, filename),
        ));

        // Line 2: "@include <always.asm>"
        tokens.push(Token::new(
            "@include".to_string(),
            TokenType::IncludeKeyword,
            TokenInfo::new("@include <always.asm>", "@include", 2, "entry_point", false, filename),
        ));

        tokens.push(Token::new(
            "<always.asm>".to_string(),
            TokenType::STDImportPath,
            TokenInfo::new("@include <always.asm>", "<always.asm>", 2, "entry_point", false, filename),
        ));

        tokens
    }
}
