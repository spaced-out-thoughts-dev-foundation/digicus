use crate::Instruction;

mod and_assign_elimination;
mod constant_propagation;
mod dead_code_elimination;
mod field_to_assign_reduction;
mod unreachable_return_elimination;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let phase_1_optimized = constant_propagation::apply(instructions);

    let phase_4_optimized = dead_code_elimination::apply(phase_1_optimized);
    let phase_5_optimized = unreachable_return_elimination::apply(phase_4_optimized);
    let phase_6_optimized = and_assign_elimination::apply(phase_5_optimized);

    phase_6_optimized
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
