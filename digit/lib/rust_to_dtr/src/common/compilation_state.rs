use crate::instruction::Instruction;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct UniqueNumberGenerator {
    pub counter: Mutex<u64>,
}

impl UniqueNumberGenerator {
    pub fn new() -> UniqueNumberGenerator {
        UniqueNumberGenerator {
            counter: Mutex::new(0),
        }
    }

    pub fn next(&self) -> u64 {
        let mut counter = self.counter.lock().unwrap();
        let result = *counter;
        *counter += 1;

        result
    }

    pub fn reset(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter = 0;
    }
}

lazy_static! {
    pub static ref GLOBAL_UNIQUE_NUMBER_GENERATOR: Arc<UniqueNumberGenerator> =
        Arc::new(UniqueNumberGenerator::new());
}

#[derive(Debug, Clone)]
pub struct CompilationState {
    pub instructions: Vec<Instruction>,
    pub scope: u32,
    pub global_uuid: Arc<UniqueNumberGenerator>,
    pub next_assignment: Option<String>,
    pub should_output: bool,
}

impl CompilationState {
    pub fn new() -> CompilationState {
        let mut result = CompilationState {
            instructions: vec![],
            scope: 0,
            global_uuid: Arc::clone(&GLOBAL_UNIQUE_NUMBER_GENERATOR),
            next_assignment: None,
            should_output: false,
        };

        result.reset_global_uuid();

        result
    }

    pub fn reset_global_uuid(&mut self) {
        self.global_uuid.reset();
    }

    pub fn update_scope(&mut self, scope: u32) {
        self.scope = scope;
    }

    pub fn update_next_assignment(&mut self, next_assignment: Option<String>) {
        self.next_assignment = next_assignment;
    }

    pub fn get_global_uuid(&self) -> u64 {
        self.global_uuid.next()
    }

    pub fn with_assignment(&self, assignment: Option<String>) -> CompilationState {
        CompilationState {
            instructions: self.instructions.clone(),
            scope: self.scope,
            global_uuid: Arc::clone(&self.global_uuid),
            next_assignment: assignment,
            should_output: self.should_output,
        }
    }

    pub fn with_scope_jump(&self, scope_jump: u32) -> CompilationState {
        CompilationState {
            instructions: self.instructions.clone(),
            scope: self.scope + scope_jump,
            global_uuid: Arc::clone(&self.global_uuid),
            next_assignment: self.next_assignment.clone(),
            should_output: self.should_output,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_number_generator() {
        let generator = UniqueNumberGenerator::new();
        assert_eq!(generator.next(), 0);
        assert_eq!(generator.next(), 1);
        assert_eq!(generator.next(), 2);

        generator.reset();
        assert_eq!(generator.next(), 0);

        generator.reset();
        assert_eq!(generator.next(), 0);
        assert_eq!(generator.next(), 1);
    }

    #[test]
    fn test_compilation_state() {
        let mut compilation_state = CompilationState::new();
        assert_eq!(compilation_state.get_global_uuid(), 0);
        assert_eq!(compilation_state.get_global_uuid(), 1);
        assert_eq!(compilation_state.get_global_uuid(), 2);

        compilation_state.update_scope(1);
        assert_eq!(compilation_state.scope, 1);

        compilation_state.update_next_assignment(Some("test".to_string()));
        assert_eq!(compilation_state.next_assignment, Some("test".to_string()));

        let new_state = compilation_state.with_assignment(Some("new".to_string()));
        assert_eq!(new_state.next_assignment, Some("new".to_string()));

        let new_state = compilation_state.with_scope_jump(1);
        assert_eq!(new_state.scope, 2);
    }
}
