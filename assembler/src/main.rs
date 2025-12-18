use std::env;

use assembler::assembler::Assembler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please specify an input file and output file");
        return;
    }

    let res = Assembler::assemble_file(&format!("asm/{}", &args[1]), &format!("bin/{}", &args[2]));

    if !res {
        println!("Unable to write");
        return;
    }
}
