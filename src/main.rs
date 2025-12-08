use crate::assembler::Assembler;

mod assembler;
mod emulator;
mod instruction;
mod lex;
mod memory;
mod registers;
mod utils;

fn main() {
    let asm = Assembler::new();
    asm.assemble_file("./asm/test.asm");
}
