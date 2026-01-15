pub mod controller;
pub mod emulator;
pub mod execute;
pub mod graphics;
pub mod memory;
pub mod registers;

#[cfg(test)]
mod emulator_tests {

    use assembler::assembler::Assembler;

    use crate::emulator::Emulator;

    fn test_file(file: &str) {
        let binary =
            Assembler::assemble_file_to_vec(&format!("../asm/tests/{}.asm", file), "../asm/std");

        if binary.is_err() {
            let err = binary.err().unwrap();
            panic!("unable to assemble file, Error: {}", err);
        }
        let binary = binary.unwrap();
        if binary.1.len() != 0 {
            for err in binary.1 {
                println!("{}", err);
            }
            panic!("Could not test because file contains errors");
        }

        let mut emu = Emulator::new();

        let res = emu.load_binary_vec(&binary.0);
        if !res {
            panic!("unable to load data_test");
        }
        emu.start(false);

        assert!(emu.registers.a == 0);
    }
    #[test]
    fn test_define() {
        test_file("define_test");
    }
    #[test]
    fn test_macros() {
        test_file("macro_test");
    }
    #[test]
    fn test_stack() {
        test_file("stack_test");
    }

    #[test]
    fn test_data() {
        test_file("data_test");
    }
    #[test]
    fn test_arithmetic() {
        test_file("arithmetic_test");
    }
    #[test]
    fn test_expression() {
        test_file("expression_test");
    }
}
