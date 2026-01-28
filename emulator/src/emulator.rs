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
    pub speed: usize,
    pub write_callbacks: Vec<Box<dyn FnMut(u16, u8, u8) + Send>>,
}

impl Emulator {
    // TODO: add print regs additional flags for debug
    pub fn new_speed(speed: usize) -> Self {
        Emulator {
            speed: speed,
            ..Default::default()
        }
    }
    pub fn new() -> Self {
        Emulator {
            speed: 0,
            ..Default::default()
        }
    }
    pub fn execute_instruction(&mut self, inst: [u8; 3]) -> u8 {
        let instruction = inst[0] >> 4;

        match instruction {
            0 => Execute::execute_mov(self, inst),
            1 => Execute::execute_ldr(self, inst),
            2 => Execute::execute_str(self, inst),
            6 => Execute::execute_jnz(self, inst),
            9 => Execute::execute_add(self, inst),
            0xA => Execute::execute_adc(self, inst),
            8 => Execute::execute_sub(self, inst),
            0xF => Execute::execute_sbb(self, inst),
            7 => Execute::execute_lsl(self, inst),
            0xB => Execute::execute_and(self, inst),
            0xC => Execute::execute_orr(self, inst),
            0xD => Execute::execute_nor(self, inst),
            0xE => Execute::execute_cmp(self, inst),
            0x5 => Execute::execute_lda(self, inst),
            0x4 => Execute::execute_pop(self, inst),
            0x3 => Execute::execute_push(self, inst),
            _ => panic!("invalid instruction"),
        }
    }
    pub fn register_callback<F: FnMut(u16, u8, u8) + Send + 'static>(&mut self, callback: F) {
        self.write_callbacks.push(Box::new(callback));
    }
    pub fn print_regs(&self, inst: [u8; 3]) {
        let dis = Disassembly::disassemble_inst(inst);
        println!(
            "{:4} {:17} {} {}",
            self.memory.get_pc(),
            dis,
            self.registers,
            self.memory.get_stack()
        );
    }
    pub fn cycle(&mut self, print_reg: bool) {
        let inst = self.memory.load_instruction();

        if print_reg {
            self.print_regs(inst);
        }

        let inst_length = self.execute_instruction(inst);

        self.memory
            .set_pc(self.memory.get_pc() + inst_length as u16);

        if self.memory.print_regs_flag() {
            self.print_regs(inst);
            self.memory[Memory::ADDITIONAL_FLAG] &= 0b11111101;
        }
        if self.speed != 0 {
            sleep(Duration::from_micros(self.speed as u64));
        }
    }
    pub fn start(&mut self, print_reg: bool) {
        loop {
            if self.memory.is_halted() {
                if print_reg {
                    println!("XX {:17} {}", "HAL", self.registers);
                }
                return;
            }
            self.cycle(print_reg);
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
