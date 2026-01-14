use regex::Regex;

use crate::utils::token::{Token, TokenType};

#[derive(PartialEq)]
pub enum TypedMacroParameter {
    Register,
    Literal,
    Both,
}

impl TypedMacroParameter {
    pub fn is_equal(self, other: TypedMacroParameter) -> bool {
        if self == TypedMacroParameter::Both || other == TypedMacroParameter::Both {
            return true;
        }
        return self == other;
    }
    const REGISTER_PARAM_EXPRESSION: &'static str = r"%r[0-9]*";
    const BOTH_PARAM_EXPRESSION: &'static str = r"%x[0-9]*";
    const LITERAL_PARAM_EXPRESSION: &'static str = r"%i[0-9]*";
    pub fn type_inst_parameter(param: &Token) -> TypedMacroParameter {
        if TokenType::LITERALS.contains(&param.kind) {
            return TypedMacroParameter::Literal;
        } else if TokenType::Register == param.kind {
            return TypedMacroParameter::Register;
        }

        panic!("this is not a valid param: {}", param);
    }

    pub fn type_inst_param_list(params: Vec<Token>) -> Vec<TypedMacroParameter> {
        params
            .iter()
            .map(|param| Self::type_inst_parameter(param))
            .collect()
    }

    pub fn type_macro_parameter(param: &Token) -> TypedMacroParameter {
        if param.kind == TokenType::MacroDefinitionParameter {
            let token = &param.token;
            if Self::check_expr(Self::BOTH_PARAM_EXPRESSION, &token) {
                return TypedMacroParameter::Both;
            } else if Self::check_expr(Self::REGISTER_PARAM_EXPRESSION, &token) {
                return TypedMacroParameter::Register;
            } else if Self::check_expr(Self::LITERAL_PARAM_EXPRESSION, &token) {
                return TypedMacroParameter::Literal;
            }
            panic!("This does not match with any known macro params: {}", token);
        } else {
            panic!("This is not a macro param: {}", param);
        }
    }

    fn check_expr(expr: &str, haystack: &str) -> bool {
        return Regex::new(expr).unwrap().is_match(haystack);
    }
}
