use std::fs;
use std::thread::sleep;
use std::time::Duration;

use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::registers::Registers;

#[derive(Default)]
pub struct Emulator {
    pub memory: Memory,
    pub registers: Registers,
}

impl Emulator {
    pub fn execute_instruction(&mut self, inst: [u8; 3]) -> u8 {
        let instruction = Instruction::parse_mnemonic(inst);

        match instruction {
            Instruction::MOV => Instruction::execute_mov(self, inst),
            Instruction::LDR => Instruction::execute_ldr(self, inst),
            Instruction::STR => Instruction::execute_str(self, inst),
            Instruction::JNZ => Instruction::execute_jnz(self, inst),
            Instruction::ADD => Instruction::execute_add(self, inst),
            Instruction::ADC => Instruction::execute_adc(self, inst),
            Instruction::SUB => Instruction::execute_sub(self, inst),
            Instruction::SBB => Instruction::execute_sbb(self, inst),
            Instruction::LSL => Instruction::execute_lsl(self, inst),
            Instruction::AND => Instruction::execute_and(self, inst),
            Instruction::ORR => Instruction::execute_orr(self, inst),
            Instruction::NOR => Instruction::execute_nor(self, inst),
            Instruction::CMP => Instruction::execute_cmp(self, inst),
            Instruction::LDA => Instruction::execute_lda(self, inst),
            Instruction::POP => Instruction::execute_pop(self, inst),
            Instruction::PUSH => Instruction::execute_push(self, inst),
        }
    }
    pub fn cycle(&mut self) {
        let inst = self.memory.load_instruction();
        let inst_length = self.execute_instruction(inst);
        self.memory
            .set_pc(self.memory.get_pc() + inst_length as u16);
    }
    pub fn start(&mut self) {
        loop {
            self.cycle();
            sleep(Duration::from_millis(100));
        }
    }
    pub fn load_binary(&mut self, filename: &str) {
        let bytes = fs::read(filename);
        if bytes.is_err() {
            println!("Unable to open file {}", filename);
            return;
        }
        let unwrapped_bytes = bytes.unwrap();
        let mut index: u16 = 0;

        unwrapped_bytes.iter().for_each(|byte| {
            self.memory[index] = *byte;
            index += 1;
        });
    }
}
