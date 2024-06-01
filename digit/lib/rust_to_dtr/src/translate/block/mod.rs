use crate::instruction::Instruction;
use crate::translate;
use core::panic;

pub fn handle_block(block: &syn::Block, scope: u32) -> Vec<Instruction> {
    let mut index = 1;
    let total_block_stmts = block.stmts.len();
    let mut instructions_to_return: Vec<Instruction> = Vec::new();
    block.stmts.iter().for_each(|stmt| {
        let assignment: Option<String> = if index == total_block_stmts {
            Some("Thing_to_return".to_string())
        } else {
            None
        };
        match translate::expression::supported::block_expression::parse_block_stmt(
            &stmt, assignment, scope,
        ) {
            Ok(block_str) => {
                block_str.iter().for_each(|instr| {
                    instructions_to_return.push(instr.clone());
                });

                if index == total_block_stmts {
                    instructions_to_return.push(Instruction::new(
                        "Return".to_string(),
                        vec!["Thing_to_return".to_string()],
                        "".to_string(),
                        scope,
                    ));
                }
            }
            Err(e) => {
                // return Err(e);
                panic!("Error: {:?}", e);
            }
        }
        index += 1;
    });

    instructions_to_return
}
