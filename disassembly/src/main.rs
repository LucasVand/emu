use std::env;

use disassembly::disassemble_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("please supply input file");
    }

    let res = disassemble_file(&format!("bin/{}", &args[1]));

    if res.is_err() {
        println!("unable to disassemble file: {}", res.unwrap_err());
    }
}
