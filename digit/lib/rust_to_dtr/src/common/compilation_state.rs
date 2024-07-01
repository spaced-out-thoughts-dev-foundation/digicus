use crate::instruction::Instruction;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopeStack {
    contents: Vec<u128>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack { contents: vec![0] }
    }

    pub fn push(&mut self, item: u128) {
        self.contents.push(item);
    }

    pub fn pop(&mut self) -> Option<u128> {
        self.contents.pop()
    }

    pub fn peek(&self) -> Option<&u128> {
        self.contents.last()
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn depth(&self) -> usize {
        self.contents.len()
    }
}

#[derive(Debug)]
pub struct UniqueNumberGenerator {
    pub counter: Mutex<u128>,
}

impl UniqueNumberGenerator {
    pub fn new() -> UniqueNumberGenerator {
        UniqueNumberGenerator {
            counter: Mutex::new(0),
        }
    }

    pub fn next(&self) -> u128 {
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
    pub scope_stack: ScopeStack,
    pub global_uuid: Arc<UniqueNumberGenerator>,
    pub next_assignment: Option<String>,
    pub should_output: bool,
    pub expression_stack: Vec<String>,
}

impl CompilationState {
    pub fn new() -> CompilationState {
        let mut result = CompilationState {
            instructions: vec![],
            scope_stack: ScopeStack::new(),
            global_uuid: Arc::clone(&GLOBAL_UNIQUE_NUMBER_GENERATOR),
            next_assignment: None,
            should_output: false,
            expression_stack: vec![],
        };

        result.reset_global_uuid();

        result
    }

    pub fn reset_global_uuid(&mut self) {
        self.global_uuid.reset();
    }

    pub fn enter_new_scope(&mut self) {
        self.scope_stack.push(self.get_global_uuid());
    }

    pub fn exit_scope(&mut self) {
        self.scope_stack.pop();
    }

    pub fn update_next_assignment(&mut self, next_assignment: Option<String>) {
        self.next_assignment = next_assignment;
    }

    pub fn get_global_uuid(&self) -> u128 {
        self.global_uuid.next()
    }

    pub fn scope(&self) -> u128 {
        *self.scope_stack.peek().unwrap()
    }

    pub fn with_assignment(&self, assignment: Option<String>) -> CompilationState {
        CompilationState {
            instructions: self.instructions.clone(),
            scope_stack: self.scope_stack.clone(),
            global_uuid: Arc::clone(&self.global_uuid),
            next_assignment: assignment,
            should_output: self.should_output,
            expression_stack: self.expression_stack.clone(),
        }
    }

    pub fn debug_state(self) {
        println!("\n###########################\n[DEBUG COMPILATION STATE]");

        let mut index = 0;
        self.expression_stack.clone().into_iter().for_each(|x| {
            println!("[{}]: {}", index, x);
            index += 1;
        });

        println!("\n###########################");
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

        compilation_state.enter_new_scope();
        assert_eq!(compilation_state.scope_stack.depth(), 2);

        compilation_state.update_next_assignment(Some("test".to_string()));
        assert_eq!(compilation_state.next_assignment, Some("test".to_string()));

        let new_state = compilation_state.with_assignment(Some("new".to_string()));
        assert_eq!(new_state.next_assignment, Some("new".to_string()));
    }

    #[test]
    fn test_stack() {
        let mut stack = ScopeStack::new();
        assert_eq!(stack.is_empty(), false);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.depth(), 3);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), Some(0));
        assert!(stack.pop().is_none());
        assert_eq!(stack.is_empty(), true);
    }
}
