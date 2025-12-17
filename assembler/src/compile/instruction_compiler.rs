use crate::utils::token::Token;
use crate::utils::token::TokenType;
use common::instruction::Instruction;

use crate::compile::compiled_token::CompiledToken;
use crate::compile::operand::Operand;
use crate::compile::parse_literal::ParseLiteral;
use std::iter::Peekable;

pub struct InstructionCompiler {}

impl InstructionCompiler {
    pub fn compile_instruction<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
        inital_token: &Token,
        compiled: &mut Vec<CompiledToken>,
    ) {
        let mut operands: Vec<Token> = Vec::new();

        while let Some(ele) = iter.peek() {
            if TokenType::INSTRUCTION_OPERANDS.contains(&ele.kind) {
                operands.push(iter.next().unwrap().clone());
            } else {
                break;
            }
        }
        let are_valid_operands = Operand::check_operands(&inital_token, &operands);
        if !are_valid_operands {
            return;
        }

        let requirements = Operand::inst_requirements(&inital_token);

        if requirements == [Operand::Register] {
            Self::encode_single_reg(&inital_token, &operands, compiled);
            return;
        }
        if requirements == [Operand::Register, Operand::Both { is_addr: false }] {
            Self::encode_reg_reglit(&inital_token, &operands, compiled);
            return;
        }
        if requirements == [Operand::Both { is_addr: false }] {
            Self::encode_reglit(&inital_token, &operands, compiled);
            return;
        }
        if requirements == [Operand::Register, Operand::Both { is_addr: true }] {
            Self::encode_reg_addr(&inital_token, &operands, compiled);
            return;
        }
        if requirements == [Operand::Both { is_addr: true }] {
            Self::encode_addr(&inital_token, &operands, compiled);
            return;
        }
        panic!("did not match any know instruction requirements");
    }
    pub fn encode_addr(
        inst_token: &Token,
        operands: &Vec<Token>,
        compiled: &mut Vec<CompiledToken>,
    ) {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;

        let is_literal = false;
        let literal: u8 = (is_literal as u8) << 3;

        let byte1 = instruction | literal;

        compiled.push(CompiledToken::create_token(byte1, &inst_token.token_info));

        if operands[0].kind == TokenType::Label {
            compiled.push(CompiledToken::create_label(
                &operands[0].token,
                &operands[0].token_info,
            ));
            return;
        }
        let doubleword = ParseLiteral::parse_u16(&operands[0]);

        let byte3 = doubleword as u8;
        let byte2 = (doubleword >> 8) as u8;

        compiled.push(CompiledToken::create_token(byte2, &operands[0].token_info));
        compiled.push(CompiledToken::create_token(byte3, &operands[0].token_info));
    }
    pub fn encode_reg_addr(
        inst_token: &Token,
        operands: &Vec<Token>,
        compiled: &mut Vec<CompiledToken>,
    ) {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;

        let is_literal = !(TokenType::Register == operands[1].kind);
        let literal: u8 = (is_literal as u8) << 3;

        let reg: u8 = Self::register_to_u8(&operands[0]);

        if is_literal {
            let byte1 = instruction | literal | reg;

            compiled.push(CompiledToken::create_token(byte1, &inst_token.token_info));

            if operands[1].kind == TokenType::Label {
                compiled.push(CompiledToken::create_label(
                    &operands[1].token,
                    &operands[1].token_info,
                ));
                return;
            }

            let doubleword: u16 = ParseLiteral::parse_u16(&operands[1]);

            let byte3 = doubleword as u8;
            let byte2 = (doubleword >> 8) as u8;

            compiled.push(CompiledToken::create_token(byte2, &operands[1].token_info));
            compiled.push(CompiledToken::create_token(byte3, &operands[1].token_info));
        } else {
            println!("This is not supported yet");
        }
    }
    pub fn encode_reglit(
        inst_token: &Token,
        operands: &Vec<Token>,
        compiled: &mut Vec<CompiledToken>,
    ) {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;

        let is_literal = !(TokenType::Register == operands[0].kind);
        let literal: u8 = (is_literal as u8) << 3;

        if is_literal {
            let byte1 = instruction | literal;

            let byte2 = ParseLiteral::parse_u8(&operands[0]);

            compiled.push(CompiledToken::create_token(byte1, &inst_token.token_info));
            compiled.push(CompiledToken::create_token(byte2, &operands[0].token_info));
        } else {
            let reg = Self::register_to_u8(&operands[0]);
            let byte1 = instruction | literal | reg;
            compiled.push(CompiledToken::create_token(byte1, &inst_token.token_info));
        }
    }
    pub fn encode_reg_reglit(
        inst_token: &Token,
        operands: &Vec<Token>,
        compiled: &mut Vec<CompiledToken>,
    ) {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;

        let is_literal = !(TokenType::Register == operands[1].kind);
        let literal: u8 = (is_literal as u8) << 3;

        let reg: u8 = Self::register_to_u8(&operands[0]);

        let byte1 = instruction | literal | reg;

        compiled.push(CompiledToken::create_token(byte1, &inst_token.token_info));

        let byte2: u8;
        if is_literal {
            // insert the value
            let value = ParseLiteral::parse_u8(&operands[1]);

            byte2 = value;
            compiled.push(CompiledToken::create_token(byte2, &operands[1].token_info));
        } else {
            byte2 = Self::register_to_u8(&operands[1]);
            compiled.push(CompiledToken::create_token(byte2, &operands[1].token_info));
        }
    }

    pub fn encode_single_reg(
        inst_token: &Token,
        operands: &Vec<Token>,
        compiled: &mut Vec<CompiledToken>,
    ) {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;
        let literal: u8 = 0;
        let reg: u8 = Self::register_to_u8(&operands[0]);

        let byte = instruction | literal | reg;

        compiled.push(CompiledToken::create_token(byte, &inst_token.token_info));
    }
    fn register_to_u8(token: &Token) -> u8 {
        let token_str: &str = &token.token;
        match token_str {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "l" => 4,
            "h" => 5,
            "z" => 6,
            "f" => 7,
            _ => panic!("not a valid register token {}", token.token),
        }
    }
}
