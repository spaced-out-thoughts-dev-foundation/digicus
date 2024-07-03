use crate::{common::compilation_state::CompilationState, Instruction};

mod and_assign_elimination;
mod constant_propagation;
mod dead_code_elimination;
mod unreachable_return_elimination;

pub fn apply(
    instructions: Vec<Instruction>,
    compilation_state: CompilationState,
) -> Vec<Instruction> {
    instructions.clone().into_iter().for_each(|instruction| {
        println!("instruction: {:?}", instruction);
    });

    let phase_1_optimized =
        constant_propagation::apply(instructions, compilation_state.scope_tree_root.clone());
    let phase_2_optimized =
        dead_code_elimination::apply(phase_1_optimized, compilation_state.scope_tree_root.clone());
    let phase_3_optimized = unreachable_return_elimination::apply(phase_2_optimized);
    let phase_4_optimized = and_assign_elimination::apply(phase_3_optimized);

    phase_4_optimized
}

pub fn create_instruction(name: &str, input: Vec<&str>, assign: &str) -> Instruction {
    Instruction::new(
        0,
        name.to_string(),
        input.into_iter().map(|s| s.to_string()).collect(),
        assign.to_string(),
        0, // does not really matter here yet... but it might in the future
    )
}

pub fn create_instruction_with_scope(
    name: &str,
    input: Vec<&str>,
    assign: &str,
    scope: u128,
) -> Instruction {
    Instruction::new(
        0,
        name.to_string(),
        input.into_iter().map(|s| s.to_string()).collect(),
        assign.to_string(),
        scope, // does not really matter here yet... but it might in the future
    )
}
