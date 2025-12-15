use crate::{
    compile::{compiled_token::CompiledToken, label_resolution::CompilerLabel},
    utils::logging::Logging,
};

pub struct LabelCompiler {}

impl LabelCompiler {
    pub fn compile_labels(compiled: &mut Vec<CompiledToken>, labels: &Vec<CompilerLabel>) {
        let mut to_add: Vec<(usize, [CompiledToken; 2])> = Vec::new();

        for (index, ele) in compiled.iter().enumerate() {
            if let CompiledToken::Label { name } = ele {
                let label = labels.iter().find(|lab| {
                    let unbracketed = name.strip_suffix("]").unwrap().strip_prefix("[").unwrap();

                    return lab.label == unbracketed;
                });

                if label.is_none() {
                    Logging::log_compiler_error_specific("unable to find label", 0, &"", &name);
                }
                let label = label.unwrap();

                to_add.push((index, CompiledToken::create_tokens(label.value)));
            }
        }

        let mut index_offset: isize = 0;

        for item in to_add {
            let index = item.0 as isize + index_offset;
            compiled.remove(index as usize);
            index_offset -= 1;

            compiled.insert(index as usize, item.1[1].clone());
            compiled.insert(index as usize, item.1[0].clone());
            index_offset += 2;
        }
    }
}
