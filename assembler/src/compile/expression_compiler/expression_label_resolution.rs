use crate::compile::{compiled_token::CompiledToken, label_resolution::CompilerLabel};

pub struct ExpressionLabelResolution {}

impl ExpressionLabelResolution {
    pub fn resolve_expression_labels(
        compiled: &mut Vec<CompiledToken>,
        labels: &Vec<CompilerLabel>,
    ) {
        compiled.iter_mut().for_each(|token| {
            if let CompiledToken::Expression {
                expr,
                double_word: _,
                info: _,
            } = token
            {
                for label in labels {
                    *expr = expr.replace(&label.label, &label.value.to_string());
                }
            }
        });
    }
}
