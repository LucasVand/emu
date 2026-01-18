use std::iter::Peekable;
use std::str::Chars;
use std::sync::LazyLock;

use regex::Regex;

use crate::lex::comment_lexer::CommentLexer;
use crate::lex::constant_lexer::ConstantLexer;
use crate::lex::expression_lexer::ExpressionLexer;
use crate::lex::identifier_lexer::IdentifierLexer;
use crate::lex::keyword_lexer::KeywordLexer;
use crate::lex::lexer_error::LexerError;
use crate::lex::lexer_error::LexerErrorType;
use crate::lex::macro_lexer::MacroLexer;
use crate::lex::register_lexer::RegisterLexer;
use crate::lex::single_lexer::SingleLexer;
use crate::utils::syntax_error::AssemblerError;
use crate::utils::token::Token;
use crate::utils::token::TokenType;
use crate::utils::token_info::TokenInfo;

pub struct Lexer {}

impl Lexer {
    pub const SEPERATOR_CHARS: &'static str = ", \n:;";

    const LEXER_FUNCTIONS: [fn(&str, char, usize, &Vec<Token>) -> Option<TokenType>; 8] = [
        KeywordLexer::parse,
        ConstantLexer::parse,
        MacroLexer::parse,
        RegisterLexer::parse,
        IdentifierLexer::parse,
        SingleLexer::parse,
        CommentLexer::parse,
        ExpressionLexer::parse,
    ];
    pub fn parse_str(file: String) -> (Vec<Token>, Vec<Box<dyn AssemblerError>>) {
        let mut parsed_tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<LexerError> = Vec::new();
        let mut line_num = 0;
        let lines: Vec<&str> = file.split("\n").collect();
        let mut char_iter = file.chars().peekable();

        while char_iter.peek().is_some() {
            let new_token = Self::next_token(&mut char_iter, &parsed_tokens, &mut line_num, &lines);
            match new_token.0 {
                Ok(token) => {
                    // we dont want add single char tokens
                    if [TokenType::SingleChar, TokenType::Comment].contains(&token.kind) {
                        continue;
                    }

                    parsed_tokens.push(token)
                }
                Err(err) => errors.push(err),
            }
        }
        // let mut iter = parsed_tokens.iter();
        // let mut num = 0;
        // let mut tokens: Vec<Token> = Vec::new();
        // while let Some(token) = iter.next() {
        //     if token.token_info.line_num == num {
        //         tokens.push(token.clone());
        //     } else {
        //         let len = tokens.len();
        //         if len != 0 {
        //             print!("{} : ", num);
        //             print!("{}", tokens[0].token_info.line);
        //             for tok in &tokens {
        //                 print!("({})", tok.kind);
        //             }
        //             println!("");
        //         }
        //         tokens.clear();
        //         num = token.token_info.line_num;
        //         tokens.push(token.clone());
        //     }
        // }
        // let len = tokens.len();
        // if len != 0 {
        //     print!("{} : ", num);
        //     print!("{}", tokens[0].token_info.line);
        //     for tok in &tokens {
        //         print!("({})", tok.kind);
        //     }
        //     println!("");
        // }

        let mappped_error = errors
            .iter()
            .map(|err| {
                return Box::<dyn AssemblerError>::from(err);
            })
            .collect();
        return (parsed_tokens, mappped_error);
    }

    // returns a the resulting token and the number of chars consumed
    fn next_token(
        iter: &mut Peekable<Chars>,
        parsed_tokens: &Vec<Token>,
        line_num: &mut usize,
        lines: &Vec<&str>,
    ) -> (Result<Token, LexerError>, usize) {
        let mut token = String::new();
        while let Some(ch) = iter.peek() {
            let trimmed_token = token.trim();
            for (index, func) in Self::LEXER_FUNCTIONS.iter().enumerate() {
                let token_type = func(&token, *ch, *line_num, parsed_tokens);
                // we found a token
                if let Some(token_type) = token_type {
                    let token_info = TokenInfo::new(
                        lines[*line_num],
                        trimmed_token,
                        *line_num,
                        &format!("Index: {}", index),
                        Self::is_address(trimmed_token),
                    );
                    let token_str = if token_type == TokenType::SingleChar {
                        &token
                    } else {
                        trimmed_token
                    };
                    let new_token = if Self::is_address(trimmed_token) {
                        Token::new_address(token_str.to_string(), token_type, token_info)
                    } else {
                        Token::new(token_str.to_string(), token_type, token_info)
                    };
                    return (Ok(new_token), token.len());
                }
            }
            if Lexer::SEPERATOR_CHARS.contains(*ch)
                && token != ""
                && !token.contains(";")
                && Self::check_brackets('(', ')', &token)
                && Self::check_brackets('[', ']', &token)
            {
                // println!("Crash Line Num: {}, Line: {}", line_num, lines[*line_num]);
                return (
                    Err(LexerError::new(
                        TokenInfo::new(
                            lines[*line_num],
                            &token,
                            *line_num,
                            "unknown",
                            Self::is_address(trimmed_token),
                        ),
                        LexerErrorType::TokenDoesNotMatch,
                    )),
                    token.len(),
                );
            }
            if *ch == '\n' {
                // println!("Token: {:?}, Line: {}", trimmed_token, lines[*line_num]);
                *line_num += 1;
            }

            token.push(
                iter.next()
                    .expect("iter has been peeked so this always exists"),
            );
        }

        return (
            Ok(Token::new(
                token.clone(),
                TokenType::EOF,
                TokenInfo::new(lines[*line_num], &token, *line_num, "unknown", false),
            )),
            token.len(),
        );
    }
    pub fn check_brackets(start: char, end: char, token: &str) -> bool {
        let mut start_count = 0;
        let mut end_count = 0;
        for ch in token.chars() {
            start_count += (ch == start) as usize;
            end_count += (ch == end) as usize;
        }
        return start_count == end_count;
    }
    pub fn is_address(token: &str) -> bool {
        static ADDRESS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\[.*\]$").unwrap());

        return ADDRESS_REGEX.is_match(token);
    }
    pub fn remove_square_brackets(token: &str) -> &str {
        return token
            .strip_prefix("[")
            .unwrap_or(token)
            .strip_suffix("]")
            .unwrap_or(token);
    }
}
