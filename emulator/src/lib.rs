pub mod emulator;
pub mod execute;
pub mod memory;
pub mod registers;

#[cfg(test)]
mod emulator_tests {
    use crate::emulator::Emulator;

    #[test]
    fn test_emulator_single_instructions() {
        let mut emu = Emulator::new();

        emu.load_binary("tests/arithmetic_test.bin");

        assert!(emu.registers.a == 0)
    }
}
