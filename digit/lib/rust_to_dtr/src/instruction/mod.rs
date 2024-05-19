#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub name: String,
    pub input: Vec<String>,
    pub assign: String,
}

impl Instruction {
    pub fn new(name: String, input: Vec<String>, assign: String) -> Self {
        Self {
            name,
            input,
            assign,
        }
    }
}
