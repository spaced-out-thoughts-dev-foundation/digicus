use crate::instruction::Instruction;
use crate::translate::expression;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprField;

pub fn handle_field_expression(
    expr: &ExprField,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    let member = match &expr.member {
        syn::Member::Named(ident) => ident.to_string(),
        syn::Member::Unnamed(index) => index.index.to_string(),
    };

    let mut base = expression::parse_expression(&*expr.base, Some("FIELD_BASE".to_string()))?;

    base.push(Instruction::new(
        "field".to_string(),
        vec!["FIELD_BASE".to_string(), member],
        assignment.unwrap_or("FIELD_RESULT".to_string()),
    ));

    Ok(base)
}
