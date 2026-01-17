use std::{env, usize};

use emulator::{
    emulator::Emulator,
    graphics::{self},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Please specifiy an input file");
        return;
    }

    let speed_str = args.get(2);
    let default_speed = "1".to_string();
    let speed_str_un = speed_str.unwrap_or(&default_speed);
    let speed = usize::from_str_radix(&speed_str_un, 10);
    if speed.is_err() {
        println!("Unable to parse speed");
        return;
    }
    let speed = speed.unwrap();

    println!("Creating Emulator");
    let print_regs = args.contains(&"-p".to_string());

    let mut emu = Emulator::new_speed(speed);

    let res = emu.load_binary(&format!("bin/{}", &args[1]));
    if !res {
        println!("Unable to load file: {}", args[1]);
        return;
    }
    if args.contains(&"-g".to_string()) {
        let _ = graphics::window::create_window(emu, print_regs);
    } else {
        emu.start(print_regs);
    }

    println!("Finished");
}
