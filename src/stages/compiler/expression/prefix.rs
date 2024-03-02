use crate::{
    code::Instruction,
    core::ast::{PrefixExpression, PrefixToken},
    stages::compiler::Compiler,
};

impl Compiler {
    pub(super) fn compile_prefix(&mut self, prefix: PrefixExpression) -> Result<(), String> {
        self.compile_expression(*prefix.right)?;

        match prefix.prefix_token {
            PrefixToken::Plus(_) => {}
            PrefixToken::Minus(_) => {
                self.instructions.push(Instruction::Negate);
            }
            PrefixToken::Bang(_) => {
                self.instructions.push(Instruction::Bang);
            }
        };

        Ok(())
    }
}
