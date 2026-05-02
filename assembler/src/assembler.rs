use std::fs;
use std::io;
use std::io::Error;

use crate::includes::entry_point::EntryPoint;
use crate::includes::include_injection::IncludeInjection;
use crate::utils::syntax_error::AssemblerError;
use crate::{
    compile::compile::Compile, lex::lexer::Lexer, preprocessor::preprocessor::Preprocessor,
};

pub struct Assembler {}

impl Assembler {
    pub fn assemble_file_to_vec(
        filename: &str,
        path_to_std: &str,
    ) -> Result<(Vec<u8>, Vec<Box<dyn AssemblerError>>), io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut error_list: Vec<Box<dyn AssemblerError>> = Vec::new();

        let (mut lexed, mut lexer_errors) = Lexer::parse_str(contents, filename);
        error_list.append(&mut lexer_errors);

        // Inject entry point tokens before the main file tokens to preserve line numbers
        let entry_point_tokens = EntryPoint::create_entry_point_tokens(filename);
        lexed = [entry_point_tokens, lexed].concat();

        let (imported, mut imported_errors) = IncludeInjection::inject_includes(lexed, path_to_std);
        error_list.append(&mut imported_errors);

        // for ele in &lexed {
        // println!("{}", ele);
        // }

        let (preprocessed, mut preprocessor_errors) = Preprocessor::preprocess_tokens(imported);

        error_list.append(&mut preprocessor_errors);

        // for ele in &preprocessed {
        //     println!("{:#?}", ele);
        // }
        let (compiled, mut compiler_errors) = Compile::compile(preprocessed);

        error_list.append(&mut compiler_errors);

        // for ele in &compiled {
        // println!("{}", ele);
        // }

        // println!("Compiled Length: {}", compiled.len());

        let mut bin: Vec<u8> = Vec::new();
        for token in compiled {
            token.compile_btyes(&mut bin);
        }

        return Ok((bin, error_list));
    }
    pub fn assemble_file(filename: &str, output: &str, path_to_std: &str) -> Result<(), io::Error> {
        let bin = Self::assemble_file_to_vec(filename, path_to_std)?;
        if bin.1.len() != 0 {
            for err in bin.1 {
                println!("{}", err);
            }
            return Err(Error::new(
                io::ErrorKind::Other,
                "File contains errors so cannot assemble",
            ));
        }
        return Ok(Self::write_file(bin.0, output)?);
    }
    fn write_file(binary: Vec<u8>, output: &str) -> Result<(), io::Error> {
        fs::write(output, binary)
    }
}
