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
            Instruction::MOV => Instruction::execute_MOV(self, inst),
            Instruction::LDR => Instruction::execute_LDR(self, inst),
            Instruction::STR => Instruction::execute_STR(self, inst),
            Instruction::JNZ => Instruction::execute_JNZ(self, inst),
            Instruction::ADD => Instruction::execute_ADD(self, inst),
            Instruction::ADC => Instruction::execute_ADC(self, inst),
            Instruction::SUB => Instruction::execute_SUB(self, inst),
            Instruction::SBB => Instruction::execute_SBB(self, inst),
            Instruction::LSL => Instruction::execute_LSL(self, inst),
            Instruction::AND => Instruction::execute_AND(self, inst),
            Instruction::ORR => Instruction::execute_ORR(self, inst),
            Instruction::NOR => Instruction::execute_NOR(self, inst),
            Instruction::CMP => Instruction::execute_CMP(self, inst),
            Instruction::LDA => Instruction::execute_LDA(self, inst),
            Instruction::POP => Instruction::execute_POP(self, inst),
            Instruction::PUSH => Instruction::execute_PUSH(self, inst),
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
