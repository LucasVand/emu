use core::panic;
use std::fmt::Display;

#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    MOV = 0x0,
    LDR = 0x1,
    STR = 0x2,
    PUSH = 0x3,
    POP = 0x4,
    LDA = 0x5,
    JNZ = 0x6,
    LSL = 0x7,
    SUB = 0x8,
    ADD = 0x9,
    ADC = 0xA,
    AND = 0xB,
    ORR = 0xC,
    NOR = 0xD,
    CMP = 0xE,
    SBB = 0xF,
}

impl Instruction {
    pub const MNEMONIC_LIST: [&'static str; 16] = [
        "mov", "ldr", "str", "push", "pop", "lda", "jnz", "lsl", "sub", "add", "adc", "and", "orr",
        "nor", "cmp", "sbb",
    ];
    pub fn operand_count(inst_str: &str) -> usize {
        let inst = Self::from_str(inst_str);
        match inst {
            Self::MOV => 2,
            Self::LDR => 2,
            Self::STR => 2,
            Self::PUSH => 1,
            Self::POP => 1,
            Self::LDA => 1,
            Self::JNZ => 1,
            Self::LSL => 2,
            Self::SUB => 2,
            Self::ADD => 2,
            Self::ADC => 2,
            Self::AND => 2,
            Self::ORR => 2,
            Self::NOR => 2,
            Self::CMP => 2,
            Self::SBB => 2,
        }
    }
    pub fn from_str(inst: &str) -> Self {
        let str_inst: &str = &inst.to_string().to_lowercase();
        match str_inst {
            "mov" => Self::MOV,
            "ldr" => Self::LDR,
            "str" => Self::STR,
            "push" => Self::PUSH,
            "pop" => Self::POP,
            "lda" => Self::LDA,
            "jnz" => Self::JNZ,
            "lsl" => Self::LSL,
            "sub" => Self::SUB,
            "add" => Self::ADD,
            "adc" => Self::ADC,
            "and" => Self::AND,
            "orr" => Self::ORR,
            "nor" => Self::NOR,
            "cmp" => Self::CMP,
            "sbb" => Self::SBB,
            _ => panic!("Invalid instruction: {}", inst),
        }
    }
    pub fn from_u8(inst: u8) -> Self {
        match inst {
            0x0 => Self::MOV,
            0x1 => Self::LDR,
            0x2 => Self::STR,
            0x3 => Self::PUSH,
            0x4 => Self::POP,
            0x5 => Self::LDA,
            0x6 => Self::JNZ,
            0x7 => Self::LSL,
            0x8 => Self::SUB,
            0x9 => Self::ADD,
            0xA => Self::ADC,
            0xB => Self::AND,
            0xC => Self::ORR,
            0xD => Self::NOR,
            0xE => Self::CMP,
            0xF => Self::SBB,
            _ => panic!("Invalid Instruction: {}", inst),
        }
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
