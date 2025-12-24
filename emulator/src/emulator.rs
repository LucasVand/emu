use crate::execute::Execute;
use crate::memory::Memory;
use crate::registers::Registers;
use common::instruction::Instruction;
use disassembly::disassemble::Disassembly;
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
    pub fn cycle(&mut self, print_reg: bool) {
        let inst = self.memory.load_instruction();
        if print_reg {
            let dis = Disassembly::disassemble_inst(inst);
            println!("{:2} {:17} {}", self.memory.get_pc(), dis, self.registers);
        }

        let inst_length = self.execute_instruction(inst);

        self.memory
            .set_pc(self.memory.get_pc() + inst_length as u16);
    }
    pub fn start(&mut self, print_reg: bool) {
        loop {
            if self.registers.is_halted() {
                if print_reg {
                    println!("XX {:17} {}", "HAL", self.registers);
                }
                return;
            }
            self.cycle(print_reg);
            sleep(Duration::from_millis(10));
        }
    }
    pub fn load_binary_vec(&mut self, bin: &Vec<u8>) -> bool {
        for (index, byte) in bin.iter().enumerate() {
            self.memory[index as u16] = *byte;
        }
        return true;
    }

    pub fn load_binary(&mut self, filename: &str) -> bool {
        let bytes = fs::read(filename);
        if bytes.is_err() {
            println!("Unable to open file {}", filename);
            return false;
        }
        let unwrapped_bytes = bytes.unwrap();
        let mut index: u16 = 0;

        unwrapped_bytes.iter().for_each(|byte| {
            self.memory[index] = *byte;
            index += 1;
        });
        return true;
    }
    pub fn clean_memory(&mut self) {
        self.memory = Memory::default();
    }
    pub fn clean_registers(&mut self) {
        self.registers = Registers::default();
    }
    pub fn clean(&mut self) {
        self.clean_memory();
        self.clean_registers();
    }
}
