use std::{env, fs::write};

use assembler::assembler::Assembler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let asm = Assembler::new();

    if args.len() < 2 {
        println!("Please specify an input file and output file");
        return;
    }

    let bin = asm.assemble_file(&format!("asm/{}", &args[1]));

    let res = write(format!("bin/{}", &args[2]), bin);

    if res.is_err() {
        println!("Unable to write");
        return;
    }
}
