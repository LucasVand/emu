use colored::Colorize;

use crate::utils::token_info::TokenInfo;

pub struct Logging {}

impl Logging {
    pub fn log_compiler_error_info(message: &str, info: &TokenInfo) {
        Self::log_error_internal(
            "Compiler",
            message,
            info.line_num,
            &info.line,
            Some(&info.token),
        );
    }
    pub fn log_lexer_error(message: &str, line_num: usize, line: &str) {
        Self::log_error_internal("Lexer", message, line_num, line, None);
    }
    pub fn log_preprocessor_error(message: &str, line_num: usize, line: &str) {
        Self::log_error_internal("Preprocessor", message, line_num, line, None);
    }

    pub fn log_preprocessor_error_info(message: &str, info: &TokenInfo) {
        Self::log_error_internal(
            "Lexer",
            message,
            info.line_num,
            &info.line,
            Some(&info.token),
        );
    }
    pub fn log_lexer_error_info(message: &str, info: &TokenInfo) {
        Self::log_error_internal(
            "Lexer",
            message,
            info.line_num,
            &info.line,
            Some(&info.token),
        );
    }
    fn log_error_internal(
        error_type: &str,
        message: &str,
        line_num: usize,
        line: &str,
        error_token: Option<&str>,
    ) {
        let error_type = format!("{} Error:", error_type);
        let main = format!("{} {}", error_type.bold().red(), message);
        println!("{}", main);

        let line_num_str = format!("-> Line: {}", line_num);
        println!("{}", line_num_str.red());

        let line = format!("{}", line);
        println!("{}", line);

        if let Some(token) = error_token {
            let loc = line.find(token);
            if let Some(location) = loc {
                let num_spaces = location - (token.len() + 2).min(location);
                let mut spaces = "".to_string();
                for _ in 0..num_spaces {
                    spaces.push_str(" ");
                }
                spaces.push_str(&format!("\"{}\"", token));
                for _ in 0..(token.len()) {
                    spaces.push_str("^");
                }
                println!("{}", spaces.red())
            } else {
                let tok_str = format!("\"{}\"", token).red();
                println!("{}", tok_str);
            }
        }

        println!("");
    }
}
