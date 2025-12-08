use regex::Regex;

use crate::lex::{
    data_lexer::DataLexer,
    define_lexer::DefineLexer,
    label_lexer::LabelLexer,
    macro_lexer::MacroLexer,
    token::{Token, TokenType},
};

pub struct Lexer {}

impl Lexer {
    pub fn parse_str(file: &str) -> Vec<Token> {
        let mut parsed_tokens: Vec<Token> = Vec::new();
        let file_string = file.to_string();
        let lines = file_string.split("\n");
        let mut line_num = 0;

        let mut error_count = 0;

        let mut num = 0;
        for line in lines.clone().into_iter() {
            num += 1;
            println!("Line {}: {}", num, line.trim());
        }
        println!("");

        for line in lines {
            line_num += 1;
            let res = Self::parse_line(line, &mut parsed_tokens, line_num);
            if !res {
                error_count += 1;
            }
        }
        println!("Total Errors: {}", error_count);
        return parsed_tokens;
    }
    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let trimmed = line.trim();

        // checks if @macro line
        if MacroLexer::check_line(line) {
            return MacroLexer::parse_line(trimmed, parsed_tokens, line_num);
        }

        //check if @define line
        if DefineLexer::check_line(trimmed) {
            return DefineLexer::parse_line(trimmed, parsed_tokens, line_num);
        }
        // checks if its a label line
        if LabelLexer::check_line(trimmed) {
            return LabelLexer::parse_line(trimmed, parsed_tokens, line_num);
        }

        // a data defining line
        if DataLexer::check_line(trimmed) {
            return DataLexer::parse_line(trimmed, parsed_tokens, line_num);
        }

        // marco defintion
        let last_token = parsed_tokens.last();
        // is last token a macro keyword
        let is_under_macro =
            last_token.is_some() && last_token.unwrap().kind == TokenType::MacroKeyword;
        // if the line starts with @macro
        let starts_with_macro = Regex::new(r"^@macro ").unwrap().is_match(trimmed);
        // is it a macro
        let is_macro = is_under_macro || starts_with_macro;
        if is_macro
            && Regex::new(
                r"^(@macro )?[a-zA-Z0-9_.]+ ((%[ri]\d+|\[%[ri]\d+\])\s*,\s*)*(%[ri]\d+|\[%[ri]\d+\]):$",
            )
            .unwrap()
            .is_match(trimmed)
        {
            // println!("Line {} is marco defintion line", line_num);
            return true;
        }
        // empty line
        if trimmed.is_empty() {
            // println!("Line {} is empty", line_num);
            return true;
        }

        // instruction line
        if Regex::new(r"^[a-zA-Z0-9_.]+ ((%[ri]\d+|\[%[ri]\d+\]|[a-zA-Z0-9._]+|\[[a-zA-Z0-9._]+\]|\(.*\))\s*,\s*)*(%[ri]\d+|\[%[ri]\d+\]|[a-zA-Z0-9._]+|\[[a-zA-Z0-9._]+\]|\(.*\))$").unwrap().is_match(trimmed) {
            // println!("Line {} is instruction", line_num);
            return true;
        }

        println!("Line {} Unable to parse unknown line", line_num);
        return false;
    }
}
