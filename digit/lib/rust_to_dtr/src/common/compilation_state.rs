use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct CompilationState {
    pub instructions: Vec<Instruction>,
    pub scope: u32,
    pub global_uuid: u32,
    pub next_assignment: Option<String>,
    pub should_output: bool,
}

impl CompilationState {
    pub fn new() -> CompilationState {
        CompilationState {
            instructions: vec![],
            scope: 0,
            global_uuid: 0,
            next_assignment: None,
            should_output: false,
        }
    }

    pub fn update_scope(&mut self, scope: u32) {
        self.scope = scope;
    }

    pub fn increment_global_uuid(&mut self) {
        self.global_uuid += 1;
    }

    pub fn update_next_assignment(&mut self, next_assignment: Option<String>) {
        self.next_assignment = next_assignment;
    }

    pub fn with_assignment(&self, assignment: Option<String>) -> CompilationState {
        CompilationState {
            instructions: self.instructions.clone(),
            scope: self.scope,
            global_uuid: self.global_uuid,
            next_assignment: assignment,
            should_output: self.should_output,
        }
    }

    pub fn with_scope_jump(&self, scope_jump: u32) -> CompilationState {
        CompilationState {
            instructions: self.instructions.clone(),
            scope: self.scope + scope_jump,
            global_uuid: self.global_uuid,
            next_assignment: self.next_assignment.clone(),
            should_output: self.should_output,
        }
    }
}
