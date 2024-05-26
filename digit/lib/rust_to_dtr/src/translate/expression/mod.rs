use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
pub mod supported;
pub mod unsupported;

fn parse_expression(
    exp: &syn::Expr,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    match exp {
        // NOT SUPPORTED
        syn::Expr::Array(array_expr) => {
            unsupported::array_expression::handle_array_expression(&array_expr)
        }
        syn::Expr::Async(async_expr) => {
            unsupported::async_expression::handle_async_expression(async_expr)
        }
        syn::Expr::Await(await_expr) => {
            unsupported::await_expression::handle_await_expression(await_expr)
        }

        syn::Expr::ForLoop(for_loop_expr) => {
            unsupported::for_loop_expression::handle_for_loop_expression(for_loop_expr)
        }
        syn::Expr::If(if_expr) => unsupported::if_expression::handle_if_expression(if_expr),
        syn::Expr::Loop(loop_expr) => {
            unsupported::loop_expression::handle_loop_expression(loop_expr)
        }
        syn::Expr::Match(match_expression) => {
            unsupported::match_expression::handle_match_expression(match_expression)
        }
        syn::Expr::Repeat(repeat_expr) => {
            unsupported::repeat_expression::handle_repeat_expression(repeat_expr)
        }
        syn::Expr::Try(try_expr) => unsupported::try_expression::handle_try_expression(try_expr),
        syn::Expr::TryBlock(try_block_expr) => {
            unsupported::try_block_expression::handle_try_block_expression(try_block_expr)
        }
        syn::Expr::Tuple(tuple_expr) => {
            unsupported::tuple_expression::handle_tuple_expression(tuple_expr)
        }
        syn::Expr::Unsafe(unsafe_expr) => {
            unsupported::unsafe_expression::handle_unsafe_expression(unsafe_expr)
        }
        syn::Expr::While(while_expr) => {
            unsupported::while_expression::handle_while_expression(while_expr)
        }
        syn::Expr::Yield(yield_expr) => {
            unsupported::yield_expression::handle_yield_expression(yield_expr)
        }

        // SUPPORTED
        syn::Expr::Binary(binary_expr) => {
            supported::binary_expression::handle_binary_expression(binary_expr, assignment)
        }
        syn::Expr::Block(block_expr) => {
            supported::block_expression::handle_block_expression(block_expr, assignment)
        }
        syn::Expr::Let(let_expr) => {
            supported::let_expression::handle_let_expression(let_expr.clone(), assignment)
        }
        syn::Expr::Lit(lit_expr) => {
            supported::lit_expression::handle_lit_expression(&lit_expr.lit, assignment)
        }
        syn::Expr::MethodCall(method_call_expr) => {
            supported::method_call_expression::handle_method_call_expression(
                method_call_expr,
                assignment,
            )
        }
        syn::Expr::Paren(paren_expr) => {
            supported::paren_expression::handle_paren_expression(paren_expr, assignment)
        }
        syn::Expr::Path(path) => {
            supported::path_expression::handle_path_expression(&path.path, assignment)
        }
        syn::Expr::Reference(reference_expr) => {
            supported::reference_expression::handle_reference_expression(reference_expr, assignment)
        }
        syn::Expr::Return(return_expr_expr) => {
            supported::return_expression::handle_return_expression(return_expr_expr, assignment)
        }
        syn::Expr::Group(group_expr) => {
            supported::group_expression::handle_group_expression(group_expr, assignment)
        }
        syn::Expr::Field(field_expr) => {
            supported::field_expression::handle_field_expression(field_expr, assignment)
        }
        syn::Expr::Assign(assign_expr) => {
            supported::assign_expression::handle_assign_expression(assign_expr, assignment)
        }
        syn::Expr::Call(call_expr) => {
            supported::call_expression::handle_call_expression(call_expr, assignment)
        }
        // NOT IMPLEMENTED //
        syn::Expr::Break(_) => Err(NotTranslatableError::Custom(
            "Break expression not supported".to_string(),
        )),

        syn::Expr::Cast(_) => Err(NotTranslatableError::Custom(
            "Cast expression not supported".to_string(),
        )),
        syn::Expr::Closure(_) => Err(NotTranslatableError::Custom(
            "Closure expression not supported".to_string(),
        )),
        syn::Expr::Const(_) => Err(NotTranslatableError::Custom(
            "Const expression not supported".to_string(),
        )),
        syn::Expr::Continue(_) => Err(NotTranslatableError::Custom(
            "Continue expression not supported".to_string(),
        )),
        syn::Expr::Infer(_) => Err(NotTranslatableError::Custom(
            "Infer expression not supported".to_string(),
        )),
        syn::Expr::Index(_) => Err(NotTranslatableError::Custom(
            "Index expression not supported".to_string(),
        )),
        syn::Expr::Macro(_) => Err(NotTranslatableError::Custom(
            "Macro expression not supported".to_string(),
        )),
        syn::Expr::Range(_) => Err(NotTranslatableError::Custom(
            "Range expression not supported".to_string(),
        )),
        syn::Expr::Struct(_) => Err(NotTranslatableError::Custom(
            "Struct expression not supported".to_string(),
        )),
        syn::Expr::Unary(_) => Err(NotTranslatableError::Custom(
            "Unary expression not supported".to_string(),
        )),
        _ => Err(NotTranslatableError::Custom(
            "Unknown expression".to_string(),
        )),
    }
}

// fn parse_macros(mac: &syn::ExprMacro) -> String {
//     let macro_itself: &syn::Macro = &mac.mac;

//     let mut macro_str = String::new();

//     // TODO: do all macros have a bang?
//     macro_str.push_str(&format!("{}!", type_name::parse_path(&macro_itself.path)));

//     macro_str.push_str(format!("{:?}", macro_itself.tokens).as_str());

//     macro_str
// }

#[cfg(test)]
mod tests {
    mod assign_expression {}
    mod break_expression {}
    mod call_expression {}
    mod cast_expression {}
    mod closure_expression {}
    mod const_expression {}
    mod continue_expression {}
    mod field_expression {}
    mod group_expression {}
    mod index_expression {}
    mod infer_expression {}
    mod macro_expression {}
    mod paren_expression {}
    mod range_expression {}
    mod reference_expression {}
    mod struct_expression {}
    mod unary_expression {}
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
        syn::BinOp::And(_)
        | syn::BinOp::Or(_)
        | syn::BinOp::BitXor(_)
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
