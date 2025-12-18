use std::{
    env,
    fs::{self, write},
};

use crate::{
    compile::{compile::Compile, compiled_token::CompiledToken},
    lex::lexer::Lexer,
    preprocessor::preprocessor::Preprocessor,
};

pub struct Assembler {}

impl Assembler {
    pub fn assemble_file(filename: &str, output: &str) -> bool {
        let contents = fs::read_to_string(filename);

        if contents.is_err() {
            println!("Unable to read file {}", filename);
            return false;
        }
        let unwrapped_contents = contents.unwrap();

        let lexed = Lexer::parse_str(&unwrapped_contents);

        // for ele in &lexed {
        //     println!("{}", ele);
        // }

        let preprocessed = Preprocessor::preprocess_tokens(&lexed);

        // for ele in &preprocessed {
        //     println!("{}", ele);
        // }

        let compiled = Compile::compile(&preprocessed);

        println!("Compiled Length: {}", compiled.len());

        let bin: Vec<u8> = compiled
            .iter()
            .map(|tok| match tok {
                CompiledToken::Binary { byte, info: _ } => {
                    return byte.clone();
                }
                CompiledToken::Label { .. } => {
                    panic!("Should not find this label: {}", tok);
                }
                CompiledToken::Expression { .. } => {
                    panic!("Should not find this expression: {}", tok);
                }
            })
            .collect();

        for ele in &bin {
            println!("{}", ele);
        }
        return Self::write_file(&bin, output);
    }
    fn write_file(binary: &Vec<u8>, output: &str) -> bool {
        let res = write(output, binary);

        if res.is_err() {
            println!("Unable to write");
            return false;
        }
        return true;
    }
}
