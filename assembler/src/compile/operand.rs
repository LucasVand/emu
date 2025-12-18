use crate::utils::logging::Logging;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use common::instruction::Instruction;

#[derive(PartialEq, Eq, Clone)]
pub enum Operand {
    Both { is_addr: bool },
    Literal { is_addr: bool },
    Register,
}
impl Operand {
    pub fn check_operands(inst_token: &Token, operands: &Vec<Token>) -> bool {
        let operands_vec: Vec<Operand> = operands
            .iter()
            .map(|token| {
                return Self::token_to_operand(token);
            })
            .collect();
        let requirements = Self::inst_requirements(&inst_token);

        if operands_vec.len() != requirements.len() {
            Logging::log_compiler_error_info(
                &format!(
                    "incorrect number of operands, expected {} but found {}",
                    requirements.len(),
                    operands_vec.len()
                ),
                &inst_token.token_info,
            );
            return false;
        }

        for i in 0..requirements.len() {
            if !requirements[i].equivalent(&operands_vec[i]) {
                Logging::log_compiler_error_info(
                    "incorrect operand found",
                    &operands[i].token_info,
                );
                return false;
            }
        }
        return true;
    }
    pub fn token_to_operand(token: &Token) -> Operand {
        if token.kind == TokenType::Register {
            return Operand::Register;
        }
        if TokenType::LITERALS.contains(&token.kind) {
            return Operand::Literal {
                is_addr: token.is_addr,
            };
        }

        panic!("Unable to match operand {}", token);
    }
    pub fn inst_requirements(inst_token: &Token) -> Vec<Operand> {
        let inst = Instruction::from_str(&inst_token.token);
        match inst {
            Instruction::MOV => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::LDR => vec![Operand::Register, Operand::Both { is_addr: true }],
            Instruction::STR => vec![Operand::Register, Operand::Both { is_addr: true }],
            Instruction::PUSH => vec![Operand::Both { is_addr: false }],
            Instruction::POP => vec![Operand::Register],
            Instruction::LDA => vec![Operand::Both { is_addr: true }],
            Instruction::JNZ => vec![Operand::Both { is_addr: false }],
            Instruction::LSL => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::SUB => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::ADD => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::ADC => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::AND => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::ORR => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::NOR => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::CMP => vec![Operand::Register, Operand::Both { is_addr: false }],
            Instruction::SBB => vec![Operand::Register, Operand::Both { is_addr: false }],
        }
    }
    pub fn equivalent(&self, other: &Operand) -> bool {
        if *self == Operand::Register && *other == Operand::Register {
            return true;
        }

        if *self == Operand::Register && *other == (Operand::Both { is_addr: false }) {
            return true;
        }

        if *other == Operand::Register && *self == (Operand::Both { is_addr: false }) {
            return true;
        }
        if *self == (Operand::Literal { is_addr: true })
            && *other == (Operand::Literal { is_addr: true })
        {
            return true;
        }
        if *self == (Operand::Literal { is_addr: false })
            && *other == (Operand::Literal { is_addr: false })
        {
            return true;
        }
        if *self == (Operand::Both { is_addr: false })
            && *other == (Operand::Literal { is_addr: false })
        {
            return true;
        }
        if *self == (Operand::Literal { is_addr: false })
            && *other == (Operand::Both { is_addr: false })
        {
            return true;
        }
        if *self == (Operand::Both { is_addr: true })
            && *other == (Operand::Literal { is_addr: true })
        {
            return true;
        }
        if *self == (Operand::Literal { is_addr: true })
            && *other == (Operand::Both { is_addr: true })
        {
            return true;
        }

        return false;
    }
    pub fn is_addr(&self) -> bool {
        match self {
            Operand::Both { is_addr } => {
                return *is_addr;
            }
            Operand::Literal { is_addr } => {
                return *is_addr;
            }
            Operand::Register => {
                return false;
            }
        }
    }
}
