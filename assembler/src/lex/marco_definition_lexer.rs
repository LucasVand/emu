use crate::lex::token::Token;
use crate::lex::token::TokenType;
use crate::utils::logging::Logging;
use regex::Regex;

pub struct MacroDefinitionLexer {}

impl MacroDefinitionLexer {
    const REGEX_EXPRESSION: &'static str =
        r"^[a-zA-Z0-9_.]+\s?((%[ri]\d+|\[%[ri]\d+\])\s*,\s*)*(%[ri]\d+|\[%[ri]\d+\])?:$";

    const MACRO_OPERAND_REGEX: &'static str = r"^(%[ri]\d+|\[%[ri]\d+\])$";

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

    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let mut sections = line.splitn(2, " ");
        let operation = sections.next();
        let operands = sections.next();

        if operation.is_none() {
            Logging::log_lexer_error("expected operation but found none", line_num, line);
            return false;
        }

        let operation = &operation.unwrap().replace(":", "");

        let operation_token = Token::new(
            operation,
            TokenType::MacroDefinitionMnemonic,
            line_num,
            line,
        );
        parsed_tokens.push(operation_token);

        if operands.is_none() {
            return true;
        }

        let operands_list = operands.unwrap().split(",");

        for ele in operands_list {
            let replaced = ele.replace(":", " ");
            let trimmed = &replaced.trim();
            if Regex::new(Self::MACRO_OPERAND_REGEX)
                .unwrap()
                .is_match(trimmed)
            {
                let token =
                    Token::new(trimmed, TokenType::MacroDefinitionParameter, line_num, line);
                parsed_tokens.push(token);
            } else {
                Logging::log_lexer_error_specific("invalid macro operand", line_num, line, ele);
                return false;
            }
        }
        return true;
    }
}
