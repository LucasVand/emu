pub mod emulator;
pub mod execute;
pub mod memory;
pub mod registers;

#[cfg(test)]
mod emulator_tests {
    use std::{env, path::Path};

    use assembler::assembler::Assembler;

    use crate::emulator::Emulator;

    #[test]
    fn test_data() {
        let res = env::set_current_dir(&Path::new("../bin/"));
        if res.is_err() {
            panic!("unable to change directory");
        }
        Assembler::assemble_file("tests/data_test.asm", "tests/data_test.bin");
        let mut emu = Emulator::new();

        let res = emu.load_binary("tests/data_test.bin");
        if !res {
            panic!("unable to load data_test");
        }
        emu.start(false);

        assert!(emu.registers.a == 0)
    }
    #[test]
    fn test_arithmetic() {
        let res = env::set_current_dir(&Path::new("../bin/"));
        if res.is_err() {
            panic!("unable to change directory");
        }
        Assembler::assemble_file("tests/arithmetic_test.asm", "tests/arithmetic_test.bin");

        let mut emu = Emulator::new();

        let res = emu.load_binary("tests/arithmetic_test.bin");
        if !res {
            panic!("unable to load arithmetic_test");
        }
        emu.start(false);

        assert!(emu.registers.a == 0);
    }
}
