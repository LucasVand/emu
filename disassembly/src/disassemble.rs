use common::instruction::Instruction;

pub struct Disassembly {}

impl Disassembly {
    const REG_BOTH_INSTRUCTIONS: [Instruction; 10] = [
        Instruction::MOV,
        Instruction::ADD,
        Instruction::ADC,
        Instruction::AND,
        Instruction::SUB,
        Instruction::SBB,
        Instruction::ORR,
        Instruction::NOR,
        Instruction::CMP,
        Instruction::LSL,
    ];
    const REG_INSTUCTIONS: [Instruction; 1] = [Instruction::POP];
    const BOTH_INSTRUCTIONS: [Instruction; 2] = [Instruction::PUSH, Instruction::JNZ];
    const REG_ADDR_INSTRUCTIONS: [Instruction; 2] = [Instruction::LDR, Instruction::STR];
    const ADDR_INSTRUCTIONS: [Instruction; 1] = [Instruction::LDA];
    pub fn disassemble_inst(inst: [u8; 3]) -> String {
        return Self::disassemble_inst_length(inst).0;
    }
    pub fn disassemble_inst_length(inst: [u8; 3]) -> (String, usize) {
        let instruction = Instruction::from_u8(inst[0] >> 4);
        let mut inst_str = Self::inst_to_str(&instruction);
        inst_str.push_str(" ");
        let length;

        if Self::REG_BOTH_INSTRUCTIONS.contains(&instruction) {
            let dis = Self::disassemble_reg_both(inst);
            length = dis.1;
            inst_str.push_str(&dis.0);
        } else if Self::REG_INSTUCTIONS.contains(&instruction) {
            let dis = Self::disassemble_reg(inst);
            length = dis.1;
            inst_str.push_str(&dis.0);
        } else if Self::REG_ADDR_INSTRUCTIONS.contains(&instruction) {
            let dis = Self::disassemble_reg_addr(inst);
            length = dis.1;
            inst_str.push_str(&dis.0);
        } else if Self::ADDR_INSTRUCTIONS.contains(&instruction) {
            let dis = Self::disassemble_addr(inst);
            length = dis.1;
            inst_str.push_str(&dis.0);
        } else if Self::BOTH_INSTRUCTIONS.contains(&instruction) {
            let dis = Self::disassemble_both(inst);
            length = dis.1;
            inst_str.push_str(&dis.0);
        } else {
            panic!("unable to disassemble instruction");
        }

        return (inst_str, length);
    }
    fn disassemble_both(inst_full: [u8; 3]) -> (String, usize) {
        let is_lit = Self::is_literal(inst_full[0]);
        let mut ret = String::new();
        let length;
        if is_lit {
            let num = inst_full[1];
            ret.push_str(&num.to_string());
            length = 2;
        } else {
            let reg = Self::reg_to_string(inst_full[0] & 0b00000111);
            ret.push_str(&reg);
            length = 1;
        }

        return (ret, length);
    }
    fn disassemble_addr(inst_full: [u8; 3]) -> (String, usize) {
        let is_lit = Self::is_literal(inst_full[0]);
        if is_lit {
            let high = inst_full[1];
            let low = inst_full[2];
            let double_word = (high as u16) << 8 | low as u16;
            return (format!("[{}]", double_word), 3);
        } else {
            let high = Self::reg_to_string(inst_full[1] >> 3);
            let low = Self::reg_to_string(inst_full[1] & 0b00000111);
            return (format!("[{}{}]", high, low), 2);
        }
    }
    fn disassemble_reg_addr(inst_full: [u8; 3]) -> (String, usize) {
        let reg1 = Self::byte_to_reg(inst_full[0]);
        let is_lit = Self::is_literal(inst_full[0]);
        if is_lit {
            let addr = (inst_full[1] as u16) << 8 | inst_full[2] as u16;
            let addr_str = format!("[{}]", addr);
            return (format!("{}, {}", reg1, addr_str), 3);
        } else {
            let high = Self::reg_to_string(inst_full[1] >> 3);

            let low = Self::reg_to_string(inst_full[1] & 0b00000111);
            return (format!("{}, [{}{}]", reg1, high, low), 2);
        }
    }

    fn disassemble_reg(inst_full: [u8; 3]) -> (String, usize) {
        let reg1 = Self::byte_to_reg(inst_full[0]);
        return (reg1.to_string(), 1);
    }
    fn disassemble_reg_both(inst_full: [u8; 3]) -> (String, usize) {
        let reg1 = Self::byte_to_reg(inst_full[0]);
        let is_lit = Self::is_literal(inst_full[0]);
        let mut ret = String::new();
        ret.push_str(&reg1);
        ret.push_str(", ");
        if is_lit {
            let num = inst_full[1];
            ret.push_str(&num.to_string());
        } else {
            let reg = Self::reg_to_string(inst_full[1]);
            ret.push_str(&reg);
        }

        return (ret, 2);
    }
    fn inst_to_str(inst: &Instruction) -> String {
        return inst.to_string();
    }
    fn is_literal(byte: u8) -> bool {
        return byte & 0b00001000 != 0;
    }
    fn byte_to_reg(byte: u8) -> String {
        let reg = byte & 0b00000111;
        return Self::reg_to_string(reg);
    }
    fn reg_to_string(reg: u8) -> String {
        match reg {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "l",
            5 => "h",
            6 => "z",
            7 => "f",
            _ => panic!("invalid register to string: {}", reg),
        }
        .to_string()
    }
}
