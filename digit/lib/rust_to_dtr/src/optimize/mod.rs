use crate::Instruction;

mod constant_propagation;
mod dead_code_elimination;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let phase_1_optimized = constant_propagation::apply(instructions);
    let phase_2_optimized = dead_code_elimination::apply(phase_1_optimized);

    phase_2_optimized
}

pub fn create_instruction(name: &str, input: Vec<&str>, assign: &str) -> Instruction {
    Instruction::new(
        name.to_string(),
        input.into_iter().map(|s| s.to_string()).collect(),
        assign.to_string(),
        0, // does not really matter here yet... but it might in the future
    )
}
