use crate::lex::data_lexer::DataLexer;
use crate::lex::define_lexer::DefineLexer;
use crate::lex::instruction_lexer::InstructionLexer;
use crate::lex::label_lexer::LabelLexer;
use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::lex::macro_lexer::MacroLexer;
use crate::lex::marco_definition_lexer::MacroDefinitionLexer;
use crate::utils::syntax_error::AssemblerError;
use crate::utils::token::Token;
use crate::utils::token_info::TokenInfo;

pub struct Lexer {}

impl Lexer {
    pub fn parse_str(file: String) -> (Vec<Token>, Vec<Box<dyn AssemblerError>>) {
        let mut parsed_tokens: Vec<Token> = Vec::new();
        let mut error: Vec<LexerError> = Vec::new();
        let file_string = file.to_string();
        let lines = file_string.split("\n");
        let mut line_num = 0;

        for line in lines {
            // remove comments
            let mut spl = line.split(";");
            // do this scary unwrap
            let ln = spl.next().unwrap();
            line_num += 1;

            let tokens_result = Self::parse_line(ln, &mut parsed_tokens, line_num);
            match tokens_result {
                Ok(mut tokens) => parsed_tokens.append(&mut tokens),
                Err(err) => error.push(err),
            }
        }
        let mappped_error = error
            .iter()
            .map(|err| {
                return Box::<dyn AssemblerError>::from(err);
            })
            .collect();
        return (parsed_tokens, mappped_error);
    }
    pub fn parse_line(
        line: &str,
        parsed_tokens: &mut Vec<Token>,
        line_num: usize,
    ) -> Result<Vec<Token>, LexerError> {
        let trimmed = line.trim();

        // checks if @macro line
        if MacroLexer::check_line(line) {
            return MacroLexer::parse_line(trimmed, line_num);
        }

        // checks if its a macro definition line
        if MacroDefinitionLexer::check_line(trimmed, parsed_tokens) {
            return MacroDefinitionLexer::parse_line(line, line_num);
        }

        //check if @define line
        if DefineLexer::check_line(trimmed) {
            return DefineLexer::parse_line(trimmed, line_num);
        }
        // checks if its a label line
        if LabelLexer::check_line(trimmed) {
            return LabelLexer::parse_line(trimmed, line_num);
        }

        // a data defining line
        if DataLexer::check_line(trimmed) {
            return DataLexer::parse_line(trimmed, line_num);
        }

        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        // instruction line
        if InstructionLexer::check_line(trimmed) {
            return InstructionLexer::parse_line(line, line_num);
        }

        return Err(LexerError::new(
            TokenInfo::new(line, line, line_num, "lexer"),
            LexerErrorType::LineDoesNotMatch,
        ));
    }
}
