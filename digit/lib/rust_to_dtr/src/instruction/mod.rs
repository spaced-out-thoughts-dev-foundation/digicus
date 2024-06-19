use crate::common::compilation_state::CompilationState;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: String,
    pub input: Vec<String>,
    pub assign: String,
    pub scope: u32,
}

impl Instruction {
    pub fn new(name: String, input: Vec<String>, assign: String, scope: u32) -> Self {
        Self {
            name,
            input,
            assign,
            scope,
        }
    }

    pub fn from_compilation_state(
        name: String,
        input: Vec<String>,
        compilation_state: &CompilationState,
    ) -> Self {
        Self {
            name,
            input,
            assign: compilation_state
                .next_assignment
                .clone()
                .unwrap_or("".to_string()),
            scope: compilation_state.scope,
        }
    }

    pub fn as_str(&self) -> String {
        if self.assign.trim() == "" {
            return format!(
                "{{ instruction: {}, input: ({}), scope: {} }}",
                self.name,
                self.input.join(", "),
                self.scope
            );
        }

        format!(
            "{{ instruction: {}, input: ({}), assign: {}, scope: {} }}",
            self.name,
            self.input.join(", "),
            self.assign,
            self.scope
        )
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.input == other.input
            && self.assign == other.assign
            && self.scope == other.scope
    }
}

impl Eq for Instruction {}
