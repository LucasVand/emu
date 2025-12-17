use crate::lex::constant_lexer::ConstantLexer;
use crate::utils::logging::Logging;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;
use common::instruction::Instruction;
use regex::Regex;

pub struct InstructionLexer {}

impl InstructionLexer {
    const REGEX_EXPRESSION: &'static str = r"^[a-zA-Z0-9_.]+\s?((%[ri]\d+|\[%[ri]\d+\]|[a-zA-Z0-9._-]+|\[[a-zA-Z0-9._]+\]|\(.*\))\s*,\s*)*(%[ri]\d+|\[%[ri]\d+\]|[a-zA-Z0-9._-]+|\[[a-zA-Z0-9._]+\]|\(.*\))?$";

    pub fn check_line(line: &str) -> bool {
        return Regex::new(Self::REGEX_EXPRESSION).unwrap().is_match(line);
    }

    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let line = line.trim_start();
        let mut sections = line.splitn(2, " ");
        let first = sections.next();

        if first.is_none() {
            Logging::log_lexer_error("expected instruction mnemonic", line_num, line);
            return false;
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
        parsed_tokens.push(mnemonic_token);

        let operands = sections.next();
        if operands.is_none() {
            return true;
        }

        let operands = operands.unwrap().trim();
        if operands.is_empty() {
            return true;
        }
        let operand_list = operands.split(",");

        for ele in operand_list {
            let ret = ConstantLexer::parse_instruction_operand(ele, line, parsed_tokens, line_num);
            if !ret {
                return false;
            }
        }

        return true;
    }
}
