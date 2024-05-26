use crate::instruction::Instruction;
use crate::{
    errors::not_translatable_error::NotTranslatableError, //, translate::expression::parse_expression,
};
use syn::ExprGroup;

pub fn handle_group_expression(
    _expr: &ExprGroup,
    _assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    Ok(vec![Instruction::new(
        "group".to_string(),
        vec!["DO_A_GROUP".to_string()],
        "DID_A_GROUP".to_string(),
    )])
}
