use crate::{ast::AstNode, object::Object};

pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,
}

pub fn compile(node: impl AstNode) -> Result<Bytecode, String> {
    let mut constants = Vec::new();

    let instructions = node.compile(|object| {
        let id = constants.len();
        constants.push(object);
        id as u32
    })?;

    Ok(Bytecode {
        instructions,
        constants,
    })
}
