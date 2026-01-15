use crate::compile::compile_error::CompilerError;
use crate::compile::operand_compiler::OperandCompiler;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use common::instruction::Instruction;

use crate::compile::compiled_token::CompiledToken;
use crate::compile::operand::Operand;
use std::iter::Peekable;
use std::vec;

pub struct InstructionCompiler {}

impl InstructionCompiler {
    pub fn compile_instruction<'a>(
        iter: &mut Peekable<vec::IntoIter<Token>>,
        inital_token: Token,
    ) -> Result<Vec<CompiledToken>, CompilerError> {
        let mut operands: Vec<Token> = Vec::new();

        while let Some(ele) = iter.peek() {
            if TokenType::INSTRUCTION_OPERANDS.contains(&ele.kind) {
                operands.push(iter.next().unwrap().clone());
            } else {
                break;
            }
        }

        Operand::check_operands(&inital_token, &operands)?;

        let requirements = Operand::inst_requirements(&inital_token);

        if requirements == [Operand::Register { is_addr: false }] {
            return Ok(Self::encode_single_reg(inital_token, operands));
        } else if requirements
            == [
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ]
        {
            return Ok(Self::encode_reg_reglit(
                inital_token,
                operands,
                requirements,
            ));
        } else if requirements == [Operand::Both { is_addr: false }] {
            return Ok(Self::encode_reglit(inital_token, operands, requirements));
        } else if requirements
            == [
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: true },
            ]
        {
            return Ok(Self::encode_reg_addr(inital_token, operands, requirements));
        } else if requirements == [Operand::Both { is_addr: true }] {
            return Ok(Self::encode_addr(inital_token, operands, requirements));
        }
        panic!("did not match any known instruction requirements");
    }
    pub fn encode_addr(
        inst_token: Token,
        operands: Vec<Token>,
        requirements: Vec<Operand>,
    ) -> Vec<CompiledToken> {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;

        let is_literal = !(TokenType::DoubleRegister == operands[0].kind);
        let literal: u8 = (is_literal as u8) << 3;

        let byte1 = instruction | literal;
        let mut compiled = Vec::new();

        compiled.push(CompiledToken::create_word(byte1, inst_token.token_info));

        let op_req = Self::create_op_req_pair(operands, requirements, 0);
        compiled.append(&mut OperandCompiler::compile_operands(op_req));
        return compiled;
    }

    pub fn encode_reg_addr(
        inst_token: Token,
        operands: Vec<Token>,
        requirements: Vec<Operand>,
    ) -> Vec<CompiledToken> {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;

        let is_literal = !(TokenType::DoubleRegister == operands[1].kind);
        let literal: u8 = (is_literal as u8) << 3;

        let reg: u8 = Self::register_to_u8(&operands[0]);
        let byte1 = instruction | literal | reg;

        let mut compiled = Vec::new();
        compiled.push(CompiledToken::create_word(byte1, inst_token.token_info));

        let op = Self::create_op_req_pair(operands, requirements, 1);
        compiled.append(&mut OperandCompiler::compile_operands(op));

        return compiled;
    }
    pub fn encode_reglit(
        inst_token: Token,
        operands: Vec<Token>,
        requirements: Vec<Operand>,
    ) -> Vec<CompiledToken> {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;

        let is_literal = !(TokenType::Register == operands[0].kind);
        let literal: u8 = (is_literal as u8) << 3;

        if is_literal {
            let byte1 = instruction | literal;
            let mut compiled = Vec::new();
            compiled.push(CompiledToken::create_word(byte1, inst_token.token_info));

            let op = Self::create_op_req_pair(operands, requirements, 0);
            compiled.append(&mut OperandCompiler::compile_operands(op));
            return compiled;
        } else {
            let reg = Self::register_to_u8(&operands[0]);
            let byte1 = instruction | literal | reg;
            return vec![CompiledToken::create_word(byte1, inst_token.token_info)];
        }
    }
    pub fn encode_reg_reglit(
        inst_token: Token,
        operands: Vec<Token>,
        requirements: Vec<Operand>,
    ) -> Vec<CompiledToken> {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;

        let is_literal = !(TokenType::Register == operands[1].kind);
        let literal: u8 = (is_literal as u8) << 3;

        let reg: u8 = Self::register_to_u8(&operands[0]);

        let byte1 = instruction | literal | reg;

        let compiled1 = CompiledToken::create_word(byte1, inst_token.token_info.clone());

        let byte2: u8;
        if is_literal {
            let op = Self::create_op_req_pair(operands, requirements, 1);
            let mut compiled = vec![compiled1];
            compiled.append(&mut OperandCompiler::compile_operands(op));
            return compiled;
        } else {
            byte2 = Self::register_to_u8(&operands[1]);
            return vec![CompiledToken::create_word(
                byte2,
                operands[1].token_info.clone(),
            )];
        }
    }

    pub fn encode_single_reg(inst_token: Token, operands: Vec<Token>) -> Vec<CompiledToken> {
        let instruction: u8 = (Instruction::from_str(&inst_token.token) as u8) << 4;
        let literal: u8 = 0;
        let reg: u8 = Self::register_to_u8(&operands[0]);

        let byte = instruction | literal | reg;

        return vec![CompiledToken::create_word(byte, inst_token.token_info)];
    }
    pub fn ch_to_u8(ch: char) -> u8 {
        match ch {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'l' => 4,
            'h' => 5,
            'z' => 6,
            'f' => 7,
            _ => panic!("not a valid register token {}", ch),
        }
    }
    pub fn register_to_u8(token: &Token) -> u8 {
        let token_str: &str = &token.token;
        let ch = token_str.chars().next().unwrap();
        return Self::ch_to_u8(ch);
    }
    fn create_op_req_pair(
        ops: Vec<Token>,
        req: Vec<Operand>,
        start: usize,
    ) -> Vec<(Token, Operand)> {
        let mut ls: Vec<(Token, Operand)> = Vec::new();
        let mut ops_iter = ops.into_iter();
        let mut req_iter = req.into_iter();
        for index in 0..ops_iter.len() {
            let op = ops_iter.next().unwrap();
            let re = req_iter.next().unwrap();
            if index >= start {
                ls.push((op, re));
            }
        }
        return ls;
    }
}
