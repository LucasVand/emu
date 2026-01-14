use colored::Colorize;
use std::{error::Error, fmt::Display};

use crate::utils::token_info::TokenInfo;

#[derive(Debug)]
pub enum AssemblerStage {
    Imports,
    Lexer,
    Preprocessor,
    Compiler,
}

impl Display for AssemblerStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Imports => "Import Error",
            Self::Lexer => "Lexer Error",
            Self::Preprocessor => "Preprocessor Error",
            Self::Compiler => "Compiler Error",
        };
        write!(f, "{}", message)
    }
}

pub trait AssemblerError {
    fn stage(&self) -> AssemblerStage;
    fn info(&self) -> TokenInfo;
    fn error(&self) -> Box<dyn Error>;
    fn fix(&self) -> Option<String>;
}

impl Display for dyn AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // format
        // <Error Type>: <message>
        // Line <line num>
        //   <Line of code where error is>
        //      ^^^^ error token
        // Fix: potential fix

        // first line
        let error_type = self.stage();
        let message = self.error();
        let info = self.info();

        let type_str = error_type.to_string().red();
        let message_str = message.to_string();
        writeln!(f, "{}: {}", type_str, message_str)?;

        // second line
        let line_str = format!("-> Line: {}", info.line_num).red();
        writeln!(f, "  | {}", line_str)?;

        writeln!(f, "  |")?;
        //third
        writeln!(f, "{} | {}", info.line_num, info.line)?;

        //forth
        if let Some(index) = info.line.find(&info.token) {
            let mut spot_str = String::new();
            for _ in 0..index {
                spot_str.push(' ');
            }
            for _ in 0..info.token.len() {
                spot_str.push('^');
            }
            spot_str.push_str(&format!("\"{}\"", &info.token));

            writeln!(f, "  | {}", spot_str.red())?;
        }
        writeln!(f, "  | ")?;

        if let Some(fix) = self.fix() {
            // fifth
            let fix_str = format!("Fix: {}", &fix).green();
            writeln!(f, "  | {}", fix_str)?;
        }

        return Ok(());
    }
}
