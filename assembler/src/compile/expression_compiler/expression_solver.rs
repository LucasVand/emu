use core::panic;
use std::iter::Peekable;

use crate::{
    compile::{
        compiled_token::CompiledToken,
        expression_compiler::{
            ast_node::{ASTBinary, ASTBinaryOp, ASTLiteral, ASTNode, ASTUnary, ASTUnaryOp},
            expression_token::{ExpressionToken, ExpressionTokenType},
        },
    },
    utils::logging::Logging,
};

pub struct ExpressionSolver {}

impl ExpressionSolver {
    // TODO: add logical operators and or xor
    pub fn sub_expressions<'a>(compiled: &mut Vec<CompiledToken>) {
        for token in compiled {
            if let CompiledToken::Expression {
                expr,
                double_word,
                info,
            } = token
            {
                let expr = expr
                    .strip_prefix("[")
                    .unwrap_or(expr)
                    .strip_suffix("]")
                    .unwrap_or(expr);

                let tokens = ExpressionToken::tokenize_expression(expr, info);
                let ast = Self::parse_expression(&mut tokens.iter().peekable(), 0);
                let value = ast.solve();

                if *double_word {
                    if (u16::MAX as isize) < value {
                        Logging::log_compiler_error_info(
                            "This value is going to be truncated",
                            info,
                        );
                    }
                    let double = value as u16;
                    *token = CompiledToken::create_double_word(double, info);
                } else {
                    if (u8::MAX as isize) < value {
                        Logging::log_compiler_error_info(
                            "This value is going to be truncated",
                            info,
                        );
                    }
                    let single = value as u8;
                    *token = CompiledToken::create_word(single, info);
                }
            }
        }
    }
    pub fn parse_expression<'a>(
        tokens: &mut Peekable<impl Iterator<Item = &'a ExpressionToken>>,
        min_binding: usize,
    ) -> Box<dyn ASTNode + 'static> {
        let token = tokens.next();
        if token.is_none() {
            panic!("ummm idk");
        }
        let token = token.unwrap();

        let mut left = token.nud(tokens);

        while tokens.peek().is_some() && min_binding < tokens.peek().unwrap().left_binding_power() {
            let token = tokens.next();
            if token.is_none() {
                panic!("ummm idk")
            }
            left = token.unwrap().led(tokens, left);
        }

        return left;
    }
}

impl ExpressionToken {
    pub fn nud<'a>(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = &'a ExpressionToken>>,
    ) -> Box<dyn ASTNode> {
        match self.kind {
            ExpressionTokenType::Decimal { num } => Box::new(ASTLiteral::new(num)),
            ExpressionTokenType::Plus => panic!("should not happen"),
            ExpressionTokenType::Multiply => panic!("should not happen"),
            ExpressionTokenType::RBrack => panic!("should not happen"),
            ExpressionTokenType::LBrack => {
                let expr = ExpressionSolver::parse_expression(tokens, 0);
                let rbrack = tokens.next();
                if rbrack.is_none() {
                    panic!("couldnt find right brack");
                }
                if rbrack.unwrap().kind != ExpressionTokenType::RBrack {
                    panic!("coudlnt find right bracket, {}", rbrack.unwrap());
                }

                return expr;
            }
            ExpressionTokenType::Divide => panic!("should not happen"),
            ExpressionTokenType::Minus => {
                let right = ExpressionSolver::parse_expression(tokens, 40);

                Box::new(ASTUnary::new(right, ASTUnaryOp::Minus))
            }
            ExpressionTokenType::RightShift => panic!("this should not happen"),
            ExpressionTokenType::LeftShift => panic!("this should not happen"),
            ExpressionTokenType::Not => {
                let right = ExpressionSolver::parse_expression(tokens, 40);
                Box::new(ASTUnary::new(right, ASTUnaryOp::Not))
            }
        }
    }
    pub fn led<'a>(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = &'a ExpressionToken>>,
        left: Box<dyn ASTNode>,
    ) -> Box<dyn ASTNode> {
        match self.kind {
            ExpressionTokenType::Plus => {
                let right = ExpressionSolver::parse_expression(tokens, self.left_binding_power());
                Box::new(ASTBinary::new(left, right, ASTBinaryOp::Plus))
            }
            ExpressionTokenType::Minus => {
                let right = ExpressionSolver::parse_expression(tokens, self.left_binding_power());
                Box::new(ASTBinary::new(left, right, ASTBinaryOp::Minus))
            }
            ExpressionTokenType::Divide => {
                let right = ExpressionSolver::parse_expression(tokens, self.left_binding_power());
                Box::new(ASTBinary::new(left, right, ASTBinaryOp::Div))
            }
            ExpressionTokenType::Multiply => {
                let right = ExpressionSolver::parse_expression(tokens, self.left_binding_power());
                Box::new(ASTBinary::new(left, right, ASTBinaryOp::Mul))
            }
            ExpressionTokenType::Decimal { num: _ } => panic!("should not happen"),
            ExpressionTokenType::LBrack => panic!("should not happen"),
            ExpressionTokenType::RBrack => panic!("should not happen"),
            ExpressionTokenType::Not => panic!("should not happen"),
            ExpressionTokenType::LeftShift => {
                let right = ExpressionSolver::parse_expression(tokens, self.left_binding_power());
                Box::new(ASTBinary::new(left, right, ASTBinaryOp::ShiftLeft))
            }
            ExpressionTokenType::RightShift => {
                let right = ExpressionSolver::parse_expression(tokens, self.left_binding_power());
                Box::new(ASTBinary::new(left, right, ASTBinaryOp::ShiftRight))
            }
        }
    }
}
