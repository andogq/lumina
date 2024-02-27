use crate::{ast::AstNode, object::Object};

pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>,
}

pub fn compile(node: impl AstNode) -> Result<Bytecode, String> {
    let mut constants = Vec::new();

    let mut register_constant = |object| {
        let id = constants.len();
        constants.push(object);
        id as u32
    };

    let instructions = node
        .compile(&mut register_constant)?
        .into_iter()
        .map(|instruction| instruction.encode())
        .flatten()
        .collect();

    Ok(Bytecode {
        instructions,
        constants,
    })
}
