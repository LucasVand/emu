use crate::compile::compiled_token::CompiledToken;
use crate::compile::data_compiler::DataCompiler;
use crate::compile::expression_compiler::expression_label_resolution::ExpressionLabelResolution;
use crate::compile::expression_compiler::expression_solver::ExpressionSolver;
use crate::compile::instruction_compiler::InstructionCompiler;
use crate::compile::label_compiler::LabelCompiler;
use crate::compile::label_resolution::CompilerLabel;
use crate::compile::label_resolution::LabelResolution;
use crate::utils::logging::Logging;
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
                    let dup = labels.iter().find(|label| {
                        return label.label == res.label;
                    });
                    if dup.is_some() {
                        Logging::log_compiler_error_info("duplicate labels found", &ele.token_info);
                    } else {
                        labels.push(res);
                    }
                }
            }
        }

        LabelCompiler::compile_labels(&mut bytes, &labels);

        ExpressionLabelResolution::resolve_expression_labels(&mut bytes, &labels);

        ExpressionSolver::sub_expressions(&mut bytes);

        return bytes;
    }
}
