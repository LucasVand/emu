use crate::compile::{compiled_token::CompiledToken, label_resolution::CompilerLabel};

pub struct ExpressionLabelResolution {}

impl ExpressionLabelResolution {
    pub fn resolve_expression_labels(
        compiled: &mut Vec<CompiledToken>,
        labels: &Vec<CompilerLabel>,
    ) {
        let mut address = 0;
        for token in compiled.iter_mut() {
            if let CompiledToken::Expression {
                expr,
                double_word: _,
                info: _,
            } = token
            {
                *expr = expr.replace("$", &(address - 1).to_string());
                for label in labels {
                    *expr = expr.replace(&label.label, &label.value.to_string());
                }
            }
            address += token.size();
        }
    }
}
