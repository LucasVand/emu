use colored::Colorize;

pub struct Logging {}

impl Logging {
    pub fn log_lexer_error(message: &str, line_num: usize, line: &str) {
        let main = format!("{} {}", "Lexer Error:".bold().red(), message);
        eprintln!("{}", main);

        let line_num_str = format!("-> Line: {}", line_num);
        eprintln!("{}", line_num_str.red());

        let line = format!("{}", line);
        eprintln!("{}", line);

        eprintln!("");
    }
}
