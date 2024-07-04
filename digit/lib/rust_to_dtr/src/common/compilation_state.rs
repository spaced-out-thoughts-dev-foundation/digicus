use crate::instruction::Instruction;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopeStack {
    // Vec<(u128, bool)> where the u128 is the scope and the bool is a flag to indicate if the scope is a loop
    contents: Vec<(u128, bool)>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            contents: vec![(0, false)],
        }
    }

    pub fn push(&mut self, item: u128, is_loop: bool) {
        self.contents.push((item, is_loop));
    }

    pub fn pop(&mut self) -> Option<(u128, bool)> {
        self.contents.pop()
    }

    pub fn just_before_last_loop_scope(&self) -> u128 {
        let mut index = self.contents.len() - 1;

        while index > 0 {
            let (_, is_loop) = self.contents[index];
            if is_loop {
                return self.contents[index - 1].0;
            }

            index -= 1;
        }

        // defaults to 0
        0
    }

    pub fn peek(&self) -> Option<&(u128, bool)> {
        self.contents.last()
    }

    pub fn peek_scope(&self) -> u128 {
        let proposed = self.contents.last().map(|(scope, _)| scope);

        match proposed {
            Some(scope) => *scope,
            // defaults to 0
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn depth(&self) -> usize {
        self.contents.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopeNaryTree {
    root: Vec<(u128, u128)>,
    uuid: char,
}

impl ScopeNaryTree {
    pub fn new(uuid: char) -> Self {
        ScopeNaryTree {
            root: vec![],
            uuid: uuid,
        }
    }

    pub fn push(&mut self, parent: u128, child: u128) {
        // first confirm that the parent is in the tree
        let mut parent_found = false;
        for (p, c) in &self.root {
            if *p == parent || *c == parent {
                parent_found = true;
                break;
            }
        }

        if !parent_found && self.root.len() > 0 {
            panic!("Parent not found in the tree");
        }

        self.root.push((parent, child));
    }

    pub fn find_parent(&self, child: u128) -> Option<u128> {
        for (parent, c) in &self.root {
            if *c == child {
                return Some(*parent);
            }
        }

        None
    }

    fn find_direct_children(&self, parent: u128) -> Vec<u128> {
        let mut children = vec![];
        for (p, c) in &self.root {
            if *p == parent {
                children.push(*c);
            }
        }

        children
    }

    pub fn max_depth(&self) -> usize {
        let mut max_depth = 0;
        let mut stack = vec![(0, 0)];

        while !stack.is_empty() {
            let (node, depth) = stack.pop().unwrap();
            if depth > max_depth {
                max_depth = depth;
            }

            let children = self.find_direct_children(node);
            for child in children {
                stack.push((child, depth + 1));
            }
        }
        max_depth + 1
    }

    pub fn is_child_of(&self, potential_child: u128, potential_parent: u128) -> bool {
        let mut parent = potential_child;
        while parent != 0 {
            match self.find_parent(parent) {
                Some(p) => parent = p,
                None => break,
            }

            if parent == potential_parent {
                return true;
            }
        }

        false
    }
}

#[test]
fn test_scope_nary_tree() {
    let mut tree = ScopeNaryTree::new('a');
    assert_eq!(tree.max_depth(), 1);

    //               0
    //             /   \
    //            1     2
    //           / \   / \
    //          3   4 5   6
    tree.push(0, 1);
    tree.push(0, 2);
    tree.push(1, 3);
    tree.push(1, 4);
    tree.push(2, 5);
    tree.push(2, 6);

    assert_eq!(tree.max_depth(), 3);
    assert_eq!(tree.find_parent(1), Some(0));
    assert_eq!(tree.find_parent(2), Some(0));
    assert_eq!(tree.find_parent(3), Some(1));
    assert_eq!(tree.find_parent(4), Some(1));

    assert_eq!(tree.find_direct_children(0), vec![1, 2]);
    assert_eq!(tree.find_direct_children(1), vec![3, 4]);
    assert_eq!(tree.find_direct_children(2), vec![5, 6]);

    assert_eq!(tree.is_child_of(3, 1), true);
    assert_eq!(tree.is_child_of(4, 1), true);
    assert_eq!(tree.is_child_of(5, 2), true);
    assert_eq!(tree.is_child_of(6, 2), true);
    assert_eq!(tree.is_child_of(1, 0), true);
    assert_eq!(tree.is_child_of(2, 0), true);
    assert_eq!(tree.is_child_of(3, 0), true);
    assert_eq!(tree.is_child_of(4, 0), true);
    assert_eq!(tree.is_child_of(1, 2), false);
    assert_eq!(tree.is_child_of(2, 1), false);
    assert_eq!(tree.is_child_of(3, 4), false);
    assert_eq!(tree.is_child_of(4, 3), false);
    assert_eq!(tree.is_child_of(5, 6), false);
    assert_eq!(tree.is_child_of(6, 5), false);
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
    pub scope_tree_root: ScopeNaryTree,
}

impl CompilationState {
    pub fn new() -> CompilationState {
        let global_uuid = Arc::clone(&GLOBAL_UNIQUE_NUMBER_GENERATOR);
        let scope_tree = ScopeNaryTree::new(rand::random::<char>());

        let mut result = CompilationState {
            instructions: vec![],
            scope_stack: ScopeStack::new(),
            global_uuid: global_uuid,
            next_assignment: None,
            should_output: false,
            expression_stack: vec![],
            scope_tree_root: scope_tree,
        };

        result.reset_global_uuid();

        result
    }

    pub fn reset_global_uuid(&mut self) {
        self.global_uuid.reset();
    }

    pub fn enter_new_scope(&mut self, is_loop: bool) {
        let parent = self.scope_stack.peek_scope();

        self.scope_stack.push(self.get_global_uuid(), is_loop);
        self.scope_tree_root
            .push(parent, self.scope_stack.peek_scope().clone());
    }

    pub fn exit_scope(&mut self) {
        self.scope_stack.pop();
    }

    pub fn update_next_assignment(&mut self, next_assignment: Option<String>) {
        self.next_assignment = next_assignment;
    }

    pub fn copy_out_current_scope_stack(&self) -> ScopeStack {
        self.scope_stack.clone()
    }

    pub fn set_scope_stack(&mut self, stack: ScopeStack) {
        self.scope_stack = stack;
    }

    pub fn get_global_uuid(&self) -> u128 {
        self.global_uuid.next()
    }

    pub fn scope(&self) -> u128 {
        self.scope_stack.peek_scope()
    }

    pub fn outside_last_loop_scope(&self) -> u128 {
        self.scope_stack.just_before_last_loop_scope()
    }

    pub fn with_assignment(&mut self, assignment: Option<String>) -> &mut CompilationState {
        self.next_assignment = assignment;

        self
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

        compilation_state.enter_new_scope(false);
        assert_eq!(compilation_state.scope_stack.depth(), 2);

        compilation_state.enter_new_scope(false);
        assert_eq!(compilation_state.scope_stack.depth(), 3);
        assert_eq!(compilation_state.scope_tree_root.max_depth(), 3);

        compilation_state.exit_scope();
        assert_eq!(compilation_state.scope_stack.depth(), 2);

        compilation_state.exit_scope();
        assert_eq!(compilation_state.scope_stack.depth(), 1);
        assert_eq!(compilation_state.scope_tree_root.max_depth(), 3);

        compilation_state.update_next_assignment(Some("test".to_string()));
        assert_eq!(compilation_state.next_assignment, Some("test".to_string()));

        let new_state = compilation_state.with_assignment(Some("new".to_string()));
        assert_eq!(new_state.next_assignment, Some("new".to_string()));
    }

    #[test]
    fn test_stack() {
        let mut stack = ScopeStack::new();
        assert_eq!(stack.is_empty(), false);
        stack.push(1, true);
        stack.push(2, false);
        assert_eq!(stack.depth(), 3);
        assert_eq!(stack.just_before_last_loop_scope(), 0);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.peek(), Some(&(2, false)));
        assert_eq!(stack.pop(), Some((2, false)));
        assert_eq!(stack.pop(), Some((1, true)));
        assert_eq!(stack.pop(), Some((0, false)));
        assert!(stack.pop().is_none());
        assert_eq!(stack.is_empty(), true);
    }
}
