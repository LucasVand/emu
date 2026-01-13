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
        let bin = Assembler::assemble_file_to_vec(&format!("../asm/tests/{}.asm", file));
        if bin.is_err() {
            panic!("unable to assemble file, Error: {}", bin.unwrap_err());
        }
        let mut emu = Emulator::new();

        let res = emu.load_binary_vec(&bin.unwrap());
        if !res {
            panic!("unable to load data_test");
        }
        emu.start(false);

        assert!(emu.registers.a == 0);
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
