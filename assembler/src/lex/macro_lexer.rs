use crate::{
    lex::lexer::Lexer,
    utils::token::{Token, TokenType},
};

pub struct MacroLexer {}

impl MacroLexer {
    const MACRO_PARAMETER: &'static str = r"^%[xri][0-9]*$";
    pub fn parse(
        token: &str,
        next: char,
        _line_num: usize,
        parsed_tokens: &Vec<Token>,
    ) -> Option<TokenType> {
        let token = token.trim();
        if !Lexer::SEPERATOR_CHARS.contains(next) {
            return None;
        }

        if Lexer::regex_match(Self::MACRO_PARAMETER, Lexer::remove_square_brackets(token)) {
            if let Some(last) = parsed_tokens.last() {
                return match last.kind {
                    TokenType::MacroDefinitionParameter => {
                        Some(TokenType::MacroDefinitionParameter)
                    }
                    TokenType::MacroDefinitionMnemonic => Some(TokenType::MacroDefinitionParameter),
                    _ => Some(TokenType::MacroParameter),
                };
            } else {
                return Some(TokenType::MacroParameter);
            }
        }

        return None;
    }
}
