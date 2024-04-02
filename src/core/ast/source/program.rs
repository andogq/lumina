use super::function::Function;

pub struct Program {
    pub functions: Vec<Function>,
    pub main: Function,
}
