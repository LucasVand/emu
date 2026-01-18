use std::env;

use assembler::assembler::Assembler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "Please specify an input file and output file, Args: {}",
            args.len()
        );
        return;
    }

    let res = Assembler::assemble_file(
        &format!("asm/{}", &args[1]),
        &format!("bin/{}", &args[2]),
        "./asm/std",
    );

    if res.is_err() {
        println!("File was not assembled");
    }
}
