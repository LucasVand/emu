use crate::compile::compiled_token::CompiledToken;
use crate::compile::data_compiler::DataCompiler;
use crate::compile::instruction_compiler::InstructionCompiler;
use crate::compile::label_compiler::LabelCompiler;
use crate::compile::label_resolution::CompilerLabel;
use crate::compile::label_resolution::LabelResolution;
use crate::utils::token::Token;
use crate::utils::token::TokenType;

pub struct Compile {}

impl Compile {
    pub fn compile(tokens: &Vec<Token>) -> Vec<CompiledToken> {
        let mut peakable = tokens.iter().peekable();
        let mut bytes: Vec<CompiledToken> = Vec::new();
        let mut labels: Vec<CompilerLabel> = Vec::new();

        let mut is_macro = false;
        while let Some(ele) = peakable.next() {
            if ele.kind == TokenType::MacroKeyword {
                is_macro = true;
            }
            if ele.kind == TokenType::EndKeyword {
                is_macro = false;
            }
            // making sure we dont compile instructions in the macro
            if !is_macro {
                if ele.kind == TokenType::Mnemonic {
                    InstructionCompiler::compile_instruction(&mut peakable, &ele, &mut bytes);
                }
                if TokenType::DATA_DEFINITIONS.contains(&ele.kind) {
                    DataCompiler::compile_data(&mut peakable, &ele, &mut bytes);
                }
                if ele.kind == TokenType::LabelDefinition {
                    let res = LabelResolution::create_compiler_label(ele, &bytes);
                    labels.push(res);
                }
            }
        }

        LabelCompiler::compile_labels(&mut bytes, &labels);

        //TODO: Add expression evaluation, with label subbing

        return bytes;
    }
}
