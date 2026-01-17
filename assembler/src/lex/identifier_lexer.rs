use common::instruction::Instruction;

use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct IdentifierLexer {}

impl IdentifierLexer {
    const IDENTIFIER_EXPR: &'static str = r"^[A-Za-z_.][A-Za-z_.0-9]*$";
    pub fn parse(
        token: &str,
        next: char,
        line_num: usize,
        parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        let token = token.trim();
        if !Lexer::SEPERATOR_CHARS.contains(next) {
            return None;
        }
        if !Lexer::regex_match(Self::IDENTIFIER_EXPR, token) {
            if Lexer::regex_match(Self::IDENTIFIER_EXPR, Lexer::remove_square_brackets(token)) {
                return Some(TokenType::Label);
            }
            return None;
        }
        if Instruction::MNEMONIC_LIST.contains(&token) {
            return Some(TokenType::Mnemonic);
        }

        // if a previous token exists
        if let Some(last) = parsed_tokens.last() {
            let token_type = match last.kind {
                TokenType::MacroKeyword => Some(TokenType::MacroDefinitionMnemonic),
                TokenType::DefineKeyword => Some(TokenType::DefineDefinitionLabel),
                TokenType::UnDefineKeyword => Some(TokenType::Label),
                _ => None,
            };
            if let Some(token_type) = token_type {
                return Some(token_type);
            }
            // check if the last token is on a different line, thus this must be a macro mnemonic
            // and next cannot be equal to colon because then it would be a label def
            if last.token_info.line_num != line_num && next != ':' {
                return Some(TokenType::MacroMnemonic);
            }
        }
        if next == ':' {
            return Some(TokenType::LabelDefinition);
        }

        // how can we differentiate a macro mnemonic and a label

        return Some(TokenType::Label);
    }
}
