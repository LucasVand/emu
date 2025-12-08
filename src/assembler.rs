use std::fs;

use crate::lex::lexer::Lexer;

pub struct Assembler {}

impl Assembler {
    const DELIMITERS: &str = " ,\n:";
    pub fn new() -> Self {
        Assembler {}
    }
    pub fn assemble_file(&self, filename: &str) {
        let contents = fs::read_to_string(filename);
        if contents.is_err() {
            println!("Unable to read file {}", filename);
            return;
        }
        let unwrapped_contents = contents.unwrap();

        let lexed = Lexer::parse_str(&unwrapped_contents);
        for ele in lexed {
            println!("{}", ele);
        }
    }
}
