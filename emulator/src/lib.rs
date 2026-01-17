pub mod controller;
pub mod emulator;
pub mod execute;
pub mod graphics;
pub mod memory;
pub mod registers;

#[cfg(test)]
mod emulator_tests {

    use std::{
        fs::{self, FileType},
        io,
    };

    use assembler::assembler::Assembler;

    use crate::emulator::Emulator;

    #[test]
    fn test_everything() -> Result<(), io::Error> {
        test_all("../asm/tests")?;
        return Ok(());
    }

    fn test_all(dir: &str) -> Result<(), io::Error> {
        let dir = fs::read_dir(dir)?;

        for item in dir {
            if let Ok(file) = item {
                if let Some(path) = file.path().to_str() {
                    if let Ok(file_type) = file.file_type() {
                        if file_type.is_dir() {
                            test_all(path)?;
                        } else {
                            test_file(path);
                        }
                    }
                }
            }
        }

        return Ok(());
    }

    fn test_file(path: &str) {
        println!("------------- Compiling ---------");
        println!("{}", path);
        let binary = Assembler::assemble_file_to_vec(path, "../asm/std");

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
        println!("------------- Compiling Complete ---------");
        let mut emu = Emulator::new();

        let res = emu.load_binary_vec(&binary.0);
        if !res {
            panic!("unable to load data_test");
        }
        emu.start(false);

        assert!(emu.registers.a == 0);
    }
}
