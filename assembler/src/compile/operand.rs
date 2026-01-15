use std::fmt::Display;

use crate::compile::compile_error::CompilerError;
use crate::compile::compile_error::CompilerErrorType;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use common::instruction::Instruction;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    Both { is_addr: bool },
    Literal { is_addr: bool },
    Register { is_addr: bool },
}
impl Operand {
    pub fn check_operands(inst_token: &Token, operands: &Vec<Token>) -> Result<(), CompilerError> {
        let operands_vec: Vec<Operand> = operands
            .iter()
            .map(|token| {
                return Self::token_to_operand(token);
            })
            .collect();
        let requirements = Self::inst_requirements(&inst_token);

        if operands_vec.len() != requirements.len() {
            return Err(CompilerError::new(
                inst_token.token_info.clone(),
                CompilerErrorType::IncorrectNumberOfOperands {
                    found: operands_vec.len(),
                    expected: requirements.len(),
                },
            ));
        }

        for i in 0..requirements.len() {
            if !requirements[i].equivalent(&operands_vec[i]) {
                return Err(CompilerError::new(
                    operands[i].token_info.clone(),
                    CompilerErrorType::IncorrectOperandFound {
                        found: operands_vec[i].clone(),
                        expected: requirements[i].clone(),
                    },
                ));
            }
        }
        return Ok(());
    }
    pub fn token_to_operand(token: &Token) -> Operand {
        if TokenType::Register == token.kind {
            return Operand::Register {
                is_addr: token.is_addr,
            };
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
            Instruction::MOV => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::LDR => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: true },
            ],
            Instruction::STR => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: true },
            ],
            Instruction::PUSH => vec![Operand::Both { is_addr: false }],
            Instruction::POP => vec![Operand::Register { is_addr: false }],
            Instruction::LDA => vec![Operand::Both { is_addr: true }],
            Instruction::JNZ => vec![Operand::Both { is_addr: false }],
            Instruction::LSL => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::SUB => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::ADD => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::ADC => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::AND => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::ORR => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::NOR => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::CMP => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
            Instruction::SBB => vec![
                Operand::Register { is_addr: false },
                Operand::Both { is_addr: false },
            ],
        }
    }
    pub fn equivalent(&self, other: &Operand) -> bool {
        if *self == (Operand::Register { is_addr: false })
            && *other == (Operand::Register { is_addr: false })
        {
            return true;
        }
        if *self == (Operand::Register { is_addr: true })
            && *other == (Operand::Register { is_addr: true })
        {
            return true;
        }
        if *self == (Operand::Register { is_addr: true })
            && *other == (Operand::Both { is_addr: true })
        {
            return true;
        }

        if *other == (Operand::Register { is_addr: true })
            && *self == (Operand::Both { is_addr: true })
        {
            return true;
        }

        if *self == (Operand::Register { is_addr: false })
            && *other == (Operand::Both { is_addr: false })
        {
            return true;
        }

        if *other == (Operand::Register { is_addr: false })
            && *self == (Operand::Both { is_addr: false })
        {
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
            Operand::Register { is_addr } => {
                return *is_addr;
            }
        }
    }
}
impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: &str = match self {
            Self::Both { is_addr } => {
                if *is_addr {
                    "Register Pair or Literal Address"
                } else {
                    "Register or Literal"
                }
            }
            Self::Literal { is_addr } => {
                if *is_addr {
                    "Literal Address"
                } else {
                    "Literal"
                }
            }
            Self::Register { is_addr } => {
                if *is_addr {
                    "Register Pair"
                } else {
                    "Register"
                }
            }
        };

        write!(f, "{}", msg)
    }
}
