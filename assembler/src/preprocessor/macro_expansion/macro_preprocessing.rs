use crate::preprocessor::macro_expansion::macro_definition::MacroDefinition;
use crate::preprocessor::macro_expansion::macro_expansion::MacroExpansion;
use crate::utils::token::Token;
use crate::utils::token::TokenType;

pub struct MacroPreprocessing {}

impl MacroPreprocessing {
    pub fn expand_macros(tokens: &mut Vec<Token>) {
        // TODO: allow for labels in macros by prepending the macro name and the invocation number
        // so that all the labels are unique after expassion
        let macro_list = MacroDefinition::create_macro_list(tokens);

        let mut iter = tokens.iter().peekable();
        let mut expansion_list: Vec<MacroExpansion> = Vec::new();
        let mut index = 0;

        while let Some(token) = iter.next() {
            // if macro then we need to expand
            if token.kind == TokenType::MacroMnemonic {
                let exp = MacroExpansion::expand_macro(&mut iter, token, &macro_list, &mut index);
                if exp.is_some() {
                    expansion_list.push(exp.unwrap());
                }
            }
            index += 1;
        }
        let mut index_offset: isize = 0;
        expansion_list.sort_by(|u, v| u.index.cmp(&v.index));

        for expansion in expansion_list {
            let index: isize = index_offset + (expansion.index as isize);
            // insert the expansions into the token list
            tokens.remove(index as usize);
            index_offset -= 1;
            for _ in 0..expansion.parameter_count {
                tokens.remove(index as usize);
                index_offset -= 1;
            }
            expansion.tokens.iter().rev().for_each(|insert_token| {
                tokens.insert(index as usize, insert_token.clone());
                index_offset += 1;
            });
        }
    }
}
