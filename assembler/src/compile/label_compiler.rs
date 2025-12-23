use crate::{
    compile::{compiled_token::CompiledToken, label_resolution::CompilerLabel},
    utils::logging::Logging,
};

pub struct LabelCompiler {}

impl LabelCompiler {
    pub fn compile_labels(compiled: &mut Vec<CompiledToken>, labels: &Vec<CompilerLabel>) {
        for ele in compiled.iter_mut() {
            if let CompiledToken::Label { name, info } = ele {
                let label = labels.iter().find(|lab| {
                    let unbracketed = name.strip_suffix("]").unwrap().strip_prefix("[").unwrap();

                    return lab.label == unbracketed;
                });

                if let Some(label) = label {
                    *ele = CompiledToken::create_double_word(label.value, info);
                } else {
                    Logging::log_compiler_error_info("unable to find label", &info);
                }
            }
        }
    }
}
