use std::env;

use emulator::emulator::Emulator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Please specifiy an input file");
        return;
    }
    println!("Creating Emulator");

    let mut emu = Emulator::new();

    emu.load_binary(&format!("bin/{}", &args[1]));
    emu.start(true);

    println!("Finished");
}
