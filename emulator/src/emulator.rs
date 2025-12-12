use crate::execute::Execute;
use crate::memory::Memory;
use crate::registers::Registers;
use common::instruction::Instruction;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

#[derive(Default)]
pub struct Emulator {
    pub memory: Memory,
    pub registers: Registers,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator::default()
    }
    pub fn execute_instruction(&mut self, inst: [u8; 3]) -> u8 {
        let instruction = Execute::parse_mnemonic(inst);

        match instruction {
            Instruction::MOV => Execute::execute_mov(self, inst),
            Instruction::LDR => Execute::execute_ldr(self, inst),
            Instruction::STR => Execute::execute_str(self, inst),
            Instruction::JNZ => Execute::execute_jnz(self, inst),
            Instruction::ADD => Execute::execute_add(self, inst),
            Instruction::ADC => Execute::execute_adc(self, inst),
            Instruction::SUB => Execute::execute_sub(self, inst),
            Instruction::SBB => Execute::execute_sbb(self, inst),
            Instruction::LSL => Execute::execute_lsl(self, inst),
            Instruction::AND => Execute::execute_and(self, inst),
            Instruction::ORR => Execute::execute_orr(self, inst),
            Instruction::NOR => Execute::execute_nor(self, inst),
            Instruction::CMP => Execute::execute_cmp(self, inst),
            Instruction::LDA => Execute::execute_lda(self, inst),
            Instruction::POP => Execute::execute_pop(self, inst),
            Instruction::PUSH => Execute::execute_push(self, inst),
        }
    }
    pub fn cycle(&mut self) {
        let inst = self.memory.load_instruction();
        let inst_length = self.execute_instruction(inst);
        self.memory
            .set_pc(self.memory.get_pc() + inst_length as u16);
    }
    pub fn start(&mut self, print_reg: bool) {
        loop {
            if print_reg {
                println!("{}", self.registers);
            }
            if self.registers.is_halted() {
                return;
            }
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
