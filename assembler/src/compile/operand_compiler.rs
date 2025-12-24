use std::fmt::Binary;

use crate::{
    compile::{
        compiled_token::CompiledToken, instruction_compiler::InstructionCompiler, operand::Operand,
        parse_literal::ParseLiteral,
    },
    utils::token::{Token, TokenType},
};

pub struct OperandCompiler {}

impl OperandCompiler {
    pub fn compile_operands(operands: &Vec<(Token, Operand)>, compiled: &mut Vec<CompiledToken>) {
        for (op, req) in operands {
            if op.kind == TokenType::Label {
                compiled.push(CompiledToken::create_label(&op.token, &op.token_info));
            } else if op.kind == TokenType::Expression {
                let is_addr = req.is_addr();
                compiled.push(CompiledToken::create_expression(
                    &op.token,
                    is_addr,
                    &op.token_info,
                ));
            } else if [TokenType::Register, TokenType::DoubleRegister].contains(&op.kind) {
                if req.is_addr() {
                    let mut chs = op.token.chars();
                    chs.next();
                    let first = chs.next().unwrap();
                    let second = chs.next().unwrap();

                    let byte1 = InstructionCompiler::ch_to_u8(first);
                    let byte2 = InstructionCompiler::ch_to_u8(second);

                    let byte = (byte1 << 3) | byte2;

                    compiled.push(CompiledToken::create_word(byte, &op.token_info));
                } else {
                    let data = InstructionCompiler::register_to_u8(&op);

                    compiled.push(CompiledToken::create_word(data, &op.token_info));
                }
            } else {
                if req.is_addr() {
                    let doubleword = ParseLiteral::parse_u16(op);
                    let byte3 = doubleword as u8;
                    let byte2 = (doubleword >> 8) as u8;

                    compiled.push(CompiledToken::create_word(byte2, &op.token_info));
                    compiled.push(CompiledToken::create_word(byte3, &op.token_info));
                } else {
                    let word = ParseLiteral::parse_u8(op);
                    compiled.push(CompiledToken::create_word(word, &op.token_info));
                }
            }
        }
    }
}
