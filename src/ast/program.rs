use crate::ast::Statement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl ToString for Program {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.to_string() + "\n")
            .collect()
    }
}
