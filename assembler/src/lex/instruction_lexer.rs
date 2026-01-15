use crate::lex::constant_lexer::ConstantLexer;
use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use common::instruction::Instruction;
use regex::Regex;

pub struct InstructionLexer {}

impl InstructionLexer {
    const REGEX_EXPRESSION: &'static str = r"^[a-zA-Z0-9_.]+\s?((%[rix]\d+|\[%[rix]\d+\]|[a-zA-Z0-9._-]+|\[[a-zA-Z0-9._]+\]|\(.*\)|\[\(.*\)\])\s*,\s*)*(%[rix]\d+|\[%[rix]\d+\]|[a-zA-Z0-9._-]+|\[[a-zA-Z0-9._]+\]|\(.*\)|\[\(.*\)\])?$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }

    pub fn parse_line(line: &str, line_num: usize) -> Result<Vec<Token>, LexerError> {
        let mut token_list: Vec<Token> = Vec::new();
        let line = line.trim_start();
        let mut sections = line.splitn(2, " ");
        let first = sections.next();

        if first.is_none() {
            return Err(LexerError::new(
                TokenInfo::new(line, line, line_num, "instruction_lexer"),
                LexerErrorType::ExpectedInstructionMnemonic,
            ));
        }

        let first = first.unwrap().trim();

        let is_basic = Instruction::MNEMONIC_LIST.contains(&first);
        let mnemonic_type = if is_basic {
            TokenType::Mnemonic
        } else {
            TokenType::MacroMnemonic
        };

        let info = TokenInfo::new(line, first, line_num, "instruction_lexer");
        let mnemonic_token = Token::new(first, mnemonic_type, info);
        token_list.push(mnemonic_token);

        let operands = sections.next();
        if operands.is_none() {
            return Ok(token_list);
        }

        let operands = operands.unwrap().trim();
        if operands.is_empty() {
            return Ok(token_list);
        }
        let operand_list = operands.split(",");

        for ele in operand_list {
            let token = ConstantLexer::parse_instruction_operand(ele, line, line_num)?;
            token_list.push(token);
        }

        return Ok(token_list);
    }
}
