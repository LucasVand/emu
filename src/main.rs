mod args;

use clap::Parser;
use std::path::PathBuf;

use assembler::Assembler;
use disassembly::disassemble_file;
use emulator::{Emulator, create_window};

use args::{AsmArgs, AsmCommand};

fn main() {
    let cli = AsmArgs::parse();

    match cli.command {
        AsmCommand::Assemble {
            input,
            output,
            std_path,
        } => {
            assemble_command(&input, &output, &std_path);
        }
        AsmCommand::Disassemble { input, detailed } => {
            disassemble_command(&input, detailed);
        }
        AsmCommand::Run {
            input,
            std_path,
            print_regs,
            graphics,
            speed,
        } => {
            run_command(&input, &std_path, print_regs, graphics, speed);
        }
        AsmCommand::Execute {
            input,
            print_regs,
            graphics,
            speed,
        } => {
            execute_command(&input, print_regs, graphics, speed);
        }
    }
}

fn assemble_command(input: &PathBuf, output: &PathBuf, std_path: &PathBuf) {
    let res = Assembler::assemble_file(
        input.to_str().unwrap_or(""),
        output.to_str().unwrap_or(""),
        std_path.to_str().unwrap_or("./asm/std"),
    );

    if let Err(e) = res {
        eprintln!("Error assembling file: {}", e);
    } else {
        println!(
            "Successfully assembled: {} -> {}",
            input.display(),
            output.display()
        );
    }
}

fn disassemble_command(input: &PathBuf, _detailed: bool) {
    let result = disassemble_file(input.to_str().unwrap_or(""));

    match result {
        Ok(_) => println!("Disassembly complete"),
        Err(e) => eprintln!("Error disassembling file: {}", e),
    }
}

fn run_command(
    input: &PathBuf,
    std_path: &PathBuf,
    print_regs: bool,
    graphics: bool,
    speed: usize,
) {
    let result = Assembler::assemble_file_to_vec(
        input.to_str().unwrap_or(""),
        std_path.to_str().unwrap_or("./asm/std"),
    );

    match result {
        Ok((binary, errors)) => {
            if !errors.is_empty() {
                for error in errors {
                    println!("{}", error);
                }
                return;
            }

            let mut emulator = Emulator::new_speed(speed);
            emulator.load_binary_vec(&binary);

            println!("Running: {}", input.display());
            if graphics {
                let _ = create_window(emulator, print_regs);
            } else {
                emulator.start(print_regs);
            }
        }
        Err(e) => {
            eprintln!("Error assembling file: {}", e);
        }
    }
}

fn execute_command(input: &PathBuf, print_regs: bool, graphics: bool, speed: usize) {
    let mut emulator = Emulator::new_speed(speed);
    let load_success = emulator.load_binary(input.to_str().unwrap_or(""));

    if !load_success {
        eprintln!("Error loading binary: {}", input.display());
        return;
    }

    println!("Executing: {}", input.display());
    if graphics {
        let _ = create_window(emulator, print_regs);
    } else {
        emulator.start(print_regs);
    }
}
