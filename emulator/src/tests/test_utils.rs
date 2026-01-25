use std::{fs, io};

use assembler::assembler::Assembler;

use crate::emulator::Emulator;

pub struct EmulatorTestUtils {}
impl EmulatorTestUtils {
    pub fn test_file(path: &str) {
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
        let mut emu = Emulator::new();

        let res = emu.load_binary_vec(&binary.0);
        if !res {
            panic!("unable to load data_test");
        }
        emu.start(false);

        assert!(emu.registers.a == 0);
    }
    pub fn test_all(dir: &str) -> Result<(), io::Error> {
        let dir = fs::read_dir(dir)?;

        for item in dir {
            if let Ok(file) = item {
                if let Some(path) = file.path().to_str() {
                    if let Ok(file_type) = file.file_type() {
                        if file_type.is_dir() {
                            Self::test_all(path)?;
                        } else {
                            if path.ends_with(".asm") {
                                Self::test_file(path);
                            }
                        }
                    }
                }
            }
        }

        return Ok(());
    }
}
