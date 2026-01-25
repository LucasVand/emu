use crate::emulator::Emulator;
use common::instruction::Instruction;

pub struct Execute {}

impl Execute {
    pub fn parse_mnemonic(inst: [u8; 3]) -> Instruction {
        let op_code = inst[0] >> 4;

        Instruction::from_u8(op_code)
    }
    pub fn first_register(inst: u8) -> u8 {
        return inst & 0b00000111;
    }
    pub fn is_literal(inst: u8) -> bool {
        return (inst & 0b00001000) == 8;
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

        emu.memory.decrement_stack();
        let sp = emu.memory.get_stack();
        emu.registers[reg1] = emu.memory[sp];
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
        let sp = emu.memory.get_stack();
        emu.memory[sp] = value;
        emu.memory.incriment_stack();

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
            let reg1 = inst[1] >> 3;
            let reg2 = inst[1] & 0b00000111;
            let value1 = emu.registers[reg1];
            let value2 = emu.registers[reg2];

            addr = (value1 as u16) << 8 | value2 as u16;
            output = 2;
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
            let reg1 = inst[1] >> 3;
            let reg2 = inst[1] & 0b00000111;
            let value1 = emu.registers[reg1];
            let value2 = emu.registers[reg2];

            addr = (value1 as u16) << 8 | value2 as u16;
            output = 2;
        }
        // call all write callbacks
        emu.write_callbacks
            .iter_mut()
            .for_each(|callback| callback(addr, emu.registers[reg], emu.memory.get_membank()));

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
            emu.memory.set_pc(emu.registers.get_hl() - output as u16);
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
        let register_value = emu.registers[reg];
        let (res, _) = emu.registers[reg].overflowing_add(value);

        emu.registers[reg] = res;

        emu.registers
            .update_carry_borrow_overflow(register_value, value, 0, true);

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
        let carry = emu.registers.get_carry() as u8;
        let register_value = emu.registers[reg];

        let (other_carry, _) = (value).overflowing_add(carry);
        let (res, _) = emu.registers[reg].overflowing_add(other_carry);

        emu.registers[reg] = res;

        emu.registers
            .update_carry_borrow_overflow(register_value, value, carry, true);

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

        let register_value = emu.registers[reg];
        let (neg_other, _) = (!value).overflowing_add(1);
        let (res, _) = emu.registers[reg].overflowing_add(neg_other);

        emu.registers[reg] = res;

        emu.registers
            .update_carry_borrow_overflow(register_value, value, 0, false);
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

        let borrow = emu.registers.get_borrow() as u8;
        let register_value = emu.registers[reg];
        let (neg_other, _) = (!value).overflowing_add(1 - borrow);
        let (res, _) = emu.registers[reg].overflowing_add(neg_other);

        emu.registers[reg] = res;

        emu.registers
            .update_carry_borrow_overflow(register_value, value, borrow, false);
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
        let res = emu.registers[reg] & value;
        emu.registers[reg] = res;
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

        let (neg_other, _) = (!other).overflowing_add(1);
        let (res, _) = value.overflowing_add(neg_other);

        emu.registers.update_zero_less(res);

        return 2;
    }
    pub fn execute_lda(emu: &mut Emulator, inst: [u8; 3]) -> u8 {
        let output;
        let literal = Self::is_literal(inst[0]);
        let addr_low: u8;
        let addr_high: u8;
        if literal {
            addr_high = inst[1];
            addr_low = inst[2];
            output = 3;
        } else {
            let reg1 = inst[1] >> 3;
            let reg2 = inst[1] & 0b00000111;
            let value1 = emu.registers[reg1];
            let value2 = emu.registers[reg2];

            addr_high = value1;
            addr_low = value2;

            output = 2;
        }
        emu.registers.h = addr_high;
        emu.registers.l = addr_low;
        return output;
    }
}
