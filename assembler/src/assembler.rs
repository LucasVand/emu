use std::fs;

use crate::{
    compile::{compile::Compile, compiled_token::CompiledToken},
    lex::lexer::Lexer,
    preprocessor::preprocessor::Preprocessor,
};

pub struct Assembler {}

impl Assembler {
    pub fn new() -> Self {
        Assembler {}
    }
    pub fn assemble_file(&self, filename: &str) -> Vec<u8> {
        let contents = fs::read_to_string(filename);
        if contents.is_err() {
            println!("Unable to read file {}", filename);
            return vec![];
        }
        let unwrapped_contents = contents.unwrap();

        let lexed = Lexer::parse_str(&unwrapped_contents);

        let preprocessed = Preprocessor::preprocess_tokens(&lexed);

        let compiled = Compile::compile(&preprocessed);
        println!("Compiled Length: {}", compiled.len());
        for ele in &compiled {
            println!("{}", ele);
        }

        let bin: Vec<u8> = compiled
            .iter()
            .map(|tok| match tok {
                CompiledToken::Binary { byte } => {
                    return byte.clone();
                }
                CompiledToken::Label { .. } => {
                    panic!("Should not find this label: {}", tok);
                }
            })
            .collect();

        return bin;
    }
}
