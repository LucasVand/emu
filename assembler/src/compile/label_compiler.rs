use crate::compile::{
    compile_error::{CompilerError, CompilerErrorType},
    compiled_token::CompiledToken,
    label_resolution::CompilerLabel,
};

pub struct LabelCompiler {}

impl LabelCompiler {
    pub fn compile_labels(
        compiled: Vec<CompiledToken>,
        labels: &Vec<CompilerLabel>,
    ) -> (Vec<CompiledToken>, Vec<CompilerError>) {
        let mut error_list = Vec::new();
        let mut new_compiled_tokens = Vec::new();

        let mut address = 0;
        for ele in compiled {
            let size = ele.size();

            if let CompiledToken::Label { name, info } = ele {
                let unbracketed = name
                    .strip_suffix("]")
                    .unwrap_or(&name)
                    .strip_prefix("[")
                    .unwrap_or(&name)
                    .trim();

                let label = labels.iter().find(|lab| {
                    return lab.label == unbracketed;
                });

                if let Some(label) = label {
                    let new_token = CompiledToken::create_double_word(label.value, info);
                    new_compiled_tokens.push(new_token);
                } else {
                    if unbracketed == "main" {
                        error_list.push(CompilerError::new(
                            info,
                            CompilerErrorType::NoMainEntryPointFound,
                        ));
                    } else {
                        error_list.push(CompilerError::new(
                            info,
                            CompilerErrorType::UnableToFindLabel,
                        ));
                    }
                }
            } else if let CompiledToken::Expression {
                expr,
                double_word,
                info,
            } = ele
            {
                let mut current_address_subbed = expr.replace("$", &(address - 1).to_string());
                for label in labels {
                    current_address_subbed =
                        current_address_subbed.replace(&label.label, &label.value.to_string());
                }

                new_compiled_tokens.push(CompiledToken::Expression {
                    expr: current_address_subbed,
                    double_word: double_word,
                    info: info,
                });
            } else {
                new_compiled_tokens.push(ele);
            }
            address += size;
        }

        return (new_compiled_tokens, error_list);
    }
}
