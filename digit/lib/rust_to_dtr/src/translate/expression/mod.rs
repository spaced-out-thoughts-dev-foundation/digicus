mod assign_expression;
mod binary_expression;
pub mod block_expression;
mod call_expression;
mod cast_expression;
mod field_expression;
mod for_loop_expression;
mod if_expression;
mod let_expression;
mod lit_expression;
mod macro_expression;
mod match_expression;
mod method_call_expression;
mod paren_expression;
mod path_expression;
mod reference_expression;
mod return_expression;
mod struct_expression;
mod tuple_expression;
mod unary_expression;

use crate::common::compilation_state;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use cast_expression::handle_cast_expression;
use unary_expression::handle_unary_expression;

pub fn parse_expression(
    exp: &syn::Expr,
    compilation_state: &mut compilation_state::CompilationState,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    match exp {
        // SUPPORTED
        syn::Expr::Binary(binary_expr) => {
            binary_expression::handle_binary_expression(binary_expr, compilation_state)
        }
        syn::Expr::Block(block_expr) => {
            block_expression::handle_block_expression(block_expr, compilation_state)
        }
        syn::Expr::Let(let_expr) => {
            let_expression::handle_let_expression(let_expr.clone(), compilation_state)
        }
        syn::Expr::Lit(lit_expr) => {
            lit_expression::handle_lit_expression(&lit_expr.lit, compilation_state)
        }
        syn::Expr::MethodCall(method_call_expr) => {
            method_call_expression::handle_method_call_expression(
                method_call_expr,
                compilation_state,
            )
        }
        syn::Expr::Paren(paren_expr) => {
            paren_expression::handle_paren_expression(paren_expr, compilation_state)
        }
        syn::Expr::Path(path) => path_expression::handle_path_expression(&path, compilation_state),
        syn::Expr::Reference(reference_expr) => {
            reference_expression::handle_reference_expression(reference_expr, compilation_state)
        }
        syn::Expr::Return(return_expr_expr) => {
            return_expression::handle_return_expression(return_expr_expr, compilation_state)
        }
        syn::Expr::Field(field_expr) => {
            field_expression::handle_field_expression(field_expr, compilation_state)
        }
        syn::Expr::Assign(assign_expr) => {
            assign_expression::handle_assign_expression(assign_expr, compilation_state)
        }
        syn::Expr::Call(call_expr) => {
            call_expression::handle_call_expression(call_expr, compilation_state)
        }
        syn::Expr::Struct(struct_expr) => {
            struct_expression::handle_struct_expression(struct_expr, compilation_state)
        }
        syn::Expr::Tuple(tuple_expr) => {
            tuple_expression::handle_tuple_expression(tuple_expr, compilation_state)
        }
        syn::Expr::If(if_expr) => if_expression::handle_if_expression(if_expr, compilation_state),
        syn::Expr::Unary(unary_expr) => handle_unary_expression(unary_expr, compilation_state),
        syn::Expr::Match(match_expression) => {
            match_expression::handle_match_expression(match_expression, compilation_state)
        }
        syn::Expr::Cast(cast_expr) => handle_cast_expression(cast_expr, compilation_state),
        syn::Expr::ForLoop(for_loop_expr) => {
            for_loop_expression::handle_for_loop_expression(for_loop_expr, compilation_state)
        }
        syn::Expr::Macro(expr_macro) => {
            macro_expression::handle_macro_expression(expr_macro, compilation_state)
        }

        _ => Err(NotTranslatableError::Custom(
            "Unsupported expression".to_string(),
        )),
    }
}

fn byte_vec_to_string(byte_vec: Vec<u8>) -> String {
    // Convert each byte into a character and collect into a string
    let characters: String = byte_vec.into_iter().map(|byte| byte as char).collect();
    characters
}

pub fn parse_lit(syn_lit: &syn::Lit) -> Result<String, NotTranslatableError> {
    match &syn_lit {
        syn::Lit::Bool(bool_lit) => Ok(bool_lit.value.to_string()),
        syn::Lit::Byte(byte_lit) => Ok(byte_lit.value().to_string()),
        syn::Lit::ByteStr(byte_str_lit) => {
            Ok(format!("{:?}", byte_vec_to_string(byte_str_lit.value())))
        }
        syn::Lit::Char(char_lit) => Ok(format!("{:?}", char_lit.value())),
        syn::Lit::Float(float_lit) => Ok(float_lit.base10_digits().to_string()),
        syn::Lit::Int(int_lit) => Ok(int_lit.base10_digits().to_string()),
        syn::Lit::Str(str_lit) => Ok(format!("{:?}", str_lit.value())),
        syn::Lit::Verbatim(verbatim_lit) => Err(NotTranslatableError::Custom(format!(
            "Verbatim literal expression: {}",
            verbatim_lit.to_string()
        ))),
        _ => Err(NotTranslatableError::Custom(
            "Unknown literal expression".to_string(),
        )),
    }
}

fn parse_binary_op(syn_bin_op: &syn::BinOp) -> Result<String, NotTranslatableError> {
    match syn_bin_op {
        syn::BinOp::Add(_) => Ok("add".to_string()),
        syn::BinOp::Sub(_) => Ok("subtract".to_string()),
        syn::BinOp::Mul(_) => Ok("multiply".to_string()),
        syn::BinOp::Div(_) => Ok("divide".to_string()),
        syn::BinOp::Rem(_) => Ok("modulo".to_string()),
        syn::BinOp::And(_) => Ok("and".to_string()),
        syn::BinOp::Or(_) => Ok("or".to_string()),
        syn::BinOp::BitXor(_)
        | syn::BinOp::Shl(_)
        | syn::BinOp::Shr(_)
        | syn::BinOp::BitXorAssign(_)
        | syn::BinOp::BitAndAssign(_)
        | syn::BinOp::BitOrAssign(_)
        | syn::BinOp::ShlAssign(_)
        | syn::BinOp::ShrAssign(_) => Err(NotTranslatableError::Custom(
            "Logical operators not supported: &, | ^ | << | >> | ^= | &= | |= | <<= | >>="
                .to_string(),
        )),
        syn::BinOp::Eq(_) => Ok("equal_to".to_string()),
        syn::BinOp::Lt(_) => Ok("less_than".to_string()),
        syn::BinOp::Le(_) => Ok("less_than_or_equal_to".to_string()),
        syn::BinOp::Ne(_) => Ok("not_equal_to".to_string()),
        syn::BinOp::Ge(_) => Ok("greater_than_or_equal_to".to_string()),
        syn::BinOp::Gt(_) => Ok("greater_than".to_string()),
        syn::BinOp::AddAssign(_) => Ok("add_and_assign".to_string()),
        syn::BinOp::SubAssign(_) => Ok("subtract_and_assign".to_string()),
        syn::BinOp::MulAssign(_) => Ok("multiply_and_assign".to_string()),
        syn::BinOp::DivAssign(_) => Ok("divide_and_assign".to_string()),
        syn::BinOp::RemAssign(_) => Ok("modulo_and_assign".to_string()),
        _ => Err(NotTranslatableError::Custom(
            "Unknown binary operator".to_string(),
        )),
    }
}

pub fn is_conditional_comparative_operator(syn_bin_op: &syn::BinOp) -> bool {
    match syn_bin_op {
        syn::BinOp::Eq(_)
        | syn::BinOp::Lt(_)
        | syn::BinOp::Le(_)
        | syn::BinOp::Ne(_)
        | syn::BinOp::Ge(_)
        | syn::BinOp::Gt(_) => true,
        _ => false,
    }
}
