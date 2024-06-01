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

    pub fn as_str(&self) -> String {
        if self.assign == "" {
            return format!(
                "{{ instruction: {}, input: ({:}), scope: {} }}",
                self.name,
                self.input.join(", "),
                self.scope
            );
        }

        format!(
            "{{ instruction: {}, input: ({:}), assign: {}, scope: {} }}",
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
