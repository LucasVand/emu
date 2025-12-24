use crate::compile::compiled_token::CompiledToken;
use crate::utils::token::Token;
use std::fmt::Display;

pub struct LabelResolution {}

#[derive(Clone, Debug)]
pub struct CompilerLabel {
    pub value: u16,
    pub label: String,
}
impl Display for CompilerLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl LabelResolution {
    pub fn create_compiler_label(
        inital_token: &Token,
        compiled: &Vec<CompiledToken>,
    ) -> CompilerLabel {
        let value = Self::find_addr(compiled);

        CompilerLabel {
            value: value as u16,
            label: inital_token.token.to_string(),
        }
    }
    fn find_addr(compiled: &Vec<CompiledToken>) -> usize {
        let mut count = 0;
        for ele in compiled {
            count += ele.size();
        }

        return count;
    }
}
