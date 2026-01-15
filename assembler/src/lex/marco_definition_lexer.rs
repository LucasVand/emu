use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use regex::Regex;

pub struct MacroDefinitionLexer {}

impl MacroDefinitionLexer {
    const REGEX_EXPRESSION: &'static str =
        r"^[a-zA-Z0-9_.]+\s?((%[rix]\d+|\[%[rix]\d+\])\s*,\s*)*(%[rix]\d+|\[%[rix]\d+\])?:$";

    const MACRO_OPERAND_REGEX: &'static str = r"^(%[rix]\d+|\[%[rix]\d+\])$";

    pub fn check_line(line: &str, parsed_tokens: &mut Vec<Token>) -> bool {
        // marco defintion
        let last_token = parsed_tokens.last();
        // is last token a macro keyword
        let is_under_macro =
            last_token.is_some() && last_token.unwrap().kind == TokenType::MacroKeyword;
        // if the line starts with @macro
        // is it a macro
        let is_macro = is_under_macro;
        return is_macro && Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }

    pub fn parse_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexerError> {
        let mut token_list: Vec<Token> = Vec::new();
        let mut sections = line.splitn(2, " ");
        let operation = sections.next();
        let operands = sections.next();

        if operation.is_none() {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "macro_definition_lexer"),
                LexerErrorType::ExpectedMacroLabel,
            ));
        }

        // remove the colon at the end
        let operation = &operation.unwrap().replace(":", "");

        let info = TokenInfo::new(line, &operation, line_num, "macro_definition_lexer");
        let operation_token = Token::new(
            operation.to_string(),
            TokenType::MacroDefinitionMnemonic,
            info,
        );
        token_list.push(operation_token);

        if operands.is_none() {
            return Ok(token_list);
        }

        let operands_list = operands.unwrap().split(",");

        // loop over all params
        for ele in operands_list {
            let replaced = ele.replace(":", " ");
            let trimmed = &replaced.trim();
            let info = TokenInfo::new(line, trimmed, line_num, "macro_definition_lexer");

            if Regex::new(Self::MACRO_OPERAND_REGEX)
                .unwrap()
                .is_match(trimmed)
            {
                let token = Token::new(
                    trimmed.to_string(),
                    TokenType::MacroDefinitionParameter,
                    info,
                );
                token_list.push(token);
            } else {
                return Err(LexerError::new(
                    info,
                    LexerErrorType::InvalidMacroDefinitionParameter,
                ));
            }
        }
        return Ok(token_list);
    }
}
