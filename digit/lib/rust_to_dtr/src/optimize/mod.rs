use crate::Instruction;

mod constant_propagation;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    constant_propagation::apply(instructions)
}
