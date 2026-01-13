use std::fs;
use std::io;

use crate::includes::includes::Includes;
use crate::{
    compile::compile::Compile, lex::lexer::Lexer, preprocessor::preprocessor::Preprocessor,
};

pub struct Assembler {}

impl Assembler {
    pub fn assemble_file_to_vec(filename: &str, path_to_std: &str) -> Result<Vec<u8>, io::Error> {
        let contents = fs::read_to_string(filename)?;

        let imports_resolved = Includes::resolve_imports(contents, path_to_std);

        let lexed = Lexer::parse_str(&imports_resolved);

        // for ele in &lexed {
        //     println!("{}", ele);
        // }

        let preprocessed = Preprocessor::preprocess_tokens(&lexed);

        // for ele in &preprocessed {
        //     println!("{}", ele);
        // }

        let compiled = Compile::compile(&preprocessed);

        // for ele in &compiled {
        //     println!("{}", ele);
        // }

        println!("Compiled Length: {}", compiled.len());

        let mut bin: Vec<u8> = Vec::new();
        for token in compiled {
            token.compile_btyes(&mut bin);
        }

        return Ok(bin);
    }
    pub fn assemble_file(filename: &str, output: &str, path_to_std: &str) -> Result<(), io::Error> {
        let bin = Self::assemble_file_to_vec(filename, path_to_std)?;
        return Ok(Self::write_file(&bin, output)?);
    }
    fn write_file(binary: &Vec<u8>, output: &str) -> Result<(), io::Error> {
        fs::write(output, binary)
    }
}
