use std::{fs, io};

use crate::disassemble::Disassembly;

pub fn disassemble_binary(bin: &Vec<u8>) {
    let mut index = 0;
    let mut inst_count = 0;
    while index < bin.len() {
        let first = bin.get(index);
        let second = bin.get(index + 1);
        let third = bin.get(index + 2);
        let inst = [first, second, third];
        let inst = inst.map(|byte| {
            if byte.is_none() {
                return 0;
            }
            return byte.unwrap().clone();
        });

        let dis = Disassembly::disassemble_inst_length(inst);
        inst_count += 1;

        print!("0x{:<4x} {:4} {:20} ", index, index, dis.0);
        for i in 0..(dis.1) {
            print!("{:0>8b} ", inst[i]);
        }
        println!("");

        index += dis.1;
    }
    println!();

    println!("Length: {} bytes", bin.len());
    println!("Instructions: {}", inst_count);
}
pub fn disassemble_file(filename: &str) -> Result<(), io::Error> {
    let bin = fs::read(filename)?;
    disassemble_binary(&bin);

    return Ok(());
}
pub fn disassemble_instruction(inst: [u8; 3]) -> String {
    Disassembly::disassemble_inst_length(inst).0
}
