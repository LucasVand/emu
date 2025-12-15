pub mod emulator;
pub mod execute;
pub mod memory;
pub mod registers;

#[cfg(test)]
mod tests {
    use crate::emulator::Emulator;

    use super::*;

    #[test]
    fn test_emulator_single_instructions() {
        let mut emu = Emulator::new();

        emu.load_binary("tests.bin");
    }
}
