use crate::common::compilation_state::CompilationState;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub id: u128,
    pub name: String,
    pub input: Vec<String>,
    pub assign: String,
    pub scope: u128,
}

impl Instruction {
    pub fn new(id: u128, name: String, input: Vec<String>, assign: String, scope: u128) -> Self {
        Self {
            id,
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
            id: compilation_state.get_global_uuid(),
            name,
            input,
            assign: compilation_state
                .next_assignment
                .clone()
                .unwrap_or("".to_string()),
            scope: compilation_state.scope(),
        }
    }

    pub fn as_str(&self) -> String {
        if self.assign.trim() == "" {
            return format!(
                "{{ id: {}, instruction: {}, input: ({}), scope: {} }}",
                self.id,
                self.name,
                self.input.join(", "),
                self.scope
            );
        }

        format!(
            "{{ id: {}, instruction: {}, input: ({}), assign: {}, scope: {} }}",
            self.id,
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
            && self.id == other.id
    }
}

impl Eq for Instruction {}
