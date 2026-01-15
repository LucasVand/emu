use assembler::assembler::Assembler;
use emulator::emulator::Emulator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Please specifiy an input file");
        return;
    }

    println!("Assembling File");
    let bin = Assembler::assemble_file_to_vec(&format!("asm/{}", &args[1]), "./asm/std");

    if bin.is_err() {
        println!("Something went wrong");
        return;
    }

    println!("Creating Emulator");
    let mut emu = Emulator::new();

    println!("Loading Binary");
    emu.load_binary_vec(&bin.unwrap());

    println!("Running");
    emu.start(true);

    println!("Finished");
    return;
}
