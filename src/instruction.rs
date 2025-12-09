use core::panic;

use crate::emulator::Emulator;

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
    pub fn parse_mnemonic(inst: [u8; 3]) -> Instruction {
        let op_code = inst[0] >> 4;

        Self::from_u8(op_code)
    }
    pub fn first_register(inst: u8) -> u8 {
        return inst & 0b00000111;
    }
    pub fn is_literal(inst: u8) -> bool {
        return (inst & 0b00001000) == 1;
    }
    pub fn execute_mov(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }
        emu.registers[reg] = value;
        return 2;
    }
    pub fn execute_pop(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg1: u8 = Self::first_register(inst[0]);

        let sp = emu.memory.get_stack();
        emu.registers[reg1] = emu.memory[sp];
        emu.memory.decrement_stack();
        return 1;
    }
    pub fn execute_push(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let output: u8;
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
            output = 2;
        } else {
            value = emu.registers[reg];
            output = 1;
        }
        emu.memory.incriment_stack();
        let sp = emu.memory.get_stack();
        emu.memory[sp] = value;

        return output;
    }
    pub fn execute_ldr(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let output: u8;
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let addr: u16;
        if literal {
            addr = ((inst[1] as u16) << 8) | (inst[2] as u16);
            output = 3;
        } else {
            addr = emu.registers.get_hl();
            output = 1;
        }

        emu.registers[reg] = emu.memory[addr];
        return output;
    }
    pub fn execute_str(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let output: u8;
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let addr: u16;
        if literal {
            addr = ((inst[1] as u16) << 8) | (inst[2] as u16);
            output = 3;
        } else {
            addr = emu.registers.get_hl();
            output = 1;
        }

        emu.memory[addr] = emu.registers[reg];
        return output;
    }
    pub fn execute_jnz(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let output: u8;
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
            output = 2;
        } else {
            value = emu.registers[reg];
            output = 1;
        }

        if value != 0 {
            emu.memory.set_pc(emu.registers.get_hl());
        }
        return output;
    }
    pub fn execute_add(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }
        let old = emu.registers[reg];
        emu.registers[reg] = emu.registers[reg] + value;
        return 2;
    }
    pub fn execute_adc(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }
        let old = emu.registers[reg];
        emu.registers[reg] = emu.registers[reg] + value; //+ emu.special.get_carry();
        return 2;
    }
    pub fn execute_sub(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }

        let old = emu.registers[reg];
        emu.registers[reg] = emu.registers[reg] - value;
        return 2;
    }
    pub fn execute_sbb(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }
        let old = emu.registers[reg];
        emu.registers[reg] = emu.registers[reg] - value; //- emu.special.get_carry();
        return 2;
    }
    pub fn execute_lsl(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }
        let old = emu.registers[reg];
        emu.registers[reg] = emu.registers[reg] << value;
        return 2;
    }
    pub fn execute_and(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }
        let old = emu.registers[reg];
        emu.registers[reg] = emu.registers[reg] & value;
        return 2;
    }
    pub fn execute_orr(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }
        emu.registers[reg] = emu.registers[reg] | value;
        return 2;
    }
    pub fn execute_nor(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let value: u8;
        if literal {
            value = inst[1];
        } else {
            value = emu.registers[inst[1]];
        }
        emu.registers[reg] = !(emu.registers[reg] | value);
        return 2;
    }
    pub fn execute_cmp(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let reg: u8 = Self::first_register(inst[0]);
        let literal: bool = Self::is_literal(inst[0]);

        let other: u8;
        if literal {
            other = inst[1];
        } else {
            other = emu.registers[inst[1]];
        }
        let value = emu.registers[reg];

        return 2;
    }
    pub fn execute_lda(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        emu.registers.h = inst[1];
        emu.registers.l = inst[2];
        return 3;
    }
}
