use crate::compile::compile_error::CompilerError;
use crate::compile::compile_error::CompilerErrorType;
use crate::compile::compiled_token::CompiledToken;
use crate::compile::data_compiler::DataCompiler;
use crate::compile::expression_compiler::expression_solver::ExpressionSolver;
use crate::compile::instruction_compiler::InstructionCompiler;
use crate::compile::label_compiler::LabelCompiler;
use crate::compile::label_resolution::CompilerLabel;
use crate::compile::label_resolution::LabelResolution;
use crate::utils::syntax_error::AssemblerError;
use crate::utils::token::Token;
use crate::utils::token::TokenType;

pub struct Compile {}

impl Compile {
    pub fn compile(tokens: Vec<Token>) -> (Vec<CompiledToken>, Vec<Box<dyn AssemblerError>>) {
        let mut error_list: Vec<CompilerError> = Vec::new();
        let mut peakable = tokens.into_iter().peekable();
        let mut bytes: Vec<CompiledToken> = Vec::new();
        let mut labels: Vec<CompilerLabel> = Vec::new();

        while let Some(ele) = peakable.next() {
            if ele.kind == TokenType::Mnemonic {
                let compiled = InstructionCompiler::compile_instruction(&mut peakable, ele);
                match compiled {
                    Ok(mut compiled) => bytes.append(&mut compiled),
                    Err(err) => error_list.push(err),
                }
            } else if TokenType::DATA_DEFINITIONS.contains(&ele.kind) {
                let (mut compiled, mut errors) = DataCompiler::compile_data(&mut peakable, ele);
                bytes.append(&mut compiled);
                error_list.append(&mut errors);
            } else if ele.kind == TokenType::LabelDefinition {
                let info = ele.token_info.clone();
                let compiler_label = LabelResolution::create_compiler_label(ele, &bytes);

                let dup = labels.iter().find(|label| {
                    return label.label == compiler_label.label;
                });
                if dup.is_some() {
                    error_list.push(CompilerError::new(
                        info,
                        CompilerErrorType::DuplicateLabelsFound,
                    ));
                } else {
                    labels.push(compiler_label);
                }
            }
        }

        let (bytes, mut errors) = LabelCompiler::compile_labels(bytes, &labels);
        error_list.append(&mut errors);

        let (bytes, mut errors) = ExpressionSolver::sub_expressions(bytes);
        error_list.append(&mut errors);

        return (
            bytes,
            error_list
                .iter()
                .map(|err| {
                    return Box::<dyn AssemblerError>::from(err);
                })
                .collect(),
        );
    }
}
