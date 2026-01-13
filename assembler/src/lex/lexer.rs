use crate::lex::data_lexer::DataLexer;
use crate::lex::define_lexer::DefineLexer;
use crate::lex::instruction_lexer::InstructionLexer;
use crate::lex::label_lexer::LabelLexer;
use crate::lex::macro_lexer::MacroLexer;
use crate::lex::marco_definition_lexer::MacroDefinitionLexer;
use crate::utils::logging::Logging;
use crate::utils::token::Token;

pub struct Lexer {}

impl Lexer {
    pub fn parse_str(file: &str) -> Vec<Token> {
        let mut parsed_tokens: Vec<Token> = Vec::new();
        let file_string = file.to_string();
        let lines = file_string.split("\n");
        let mut line_num = 0;

        for line in lines {
            // remove comments
            let mut spl = line.split(";");
            // do this unsafe unwrap
            let ln = spl.next().unwrap();
            line_num += 1;
            Self::parse_line(ln, &mut parsed_tokens, line_num);
        }
        return parsed_tokens;
    }
    pub fn parse_line(line: &str, parsed_tokens: &mut Vec<Token>, line_num: usize) -> bool {
        let trimmed = line.trim();

        // checks if @macro line
        if MacroLexer::check_line(line) {
            return MacroLexer::parse_line(trimmed, parsed_tokens, line_num);
        }

        // checks if its a macro definition line
        if MacroDefinitionLexer::check_line(trimmed, parsed_tokens) {
            return MacroDefinitionLexer::parse_line(line, parsed_tokens, line_num);
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

        if trimmed.is_empty() {
            return true;
        }

        // instruction line
        if InstructionLexer::check_line(trimmed) {
            return InstructionLexer::parse_line(line, parsed_tokens, line_num);
        }

        Logging::log_lexer_error("line does not match any known expression", line_num, line);
        return false;
    }
}
