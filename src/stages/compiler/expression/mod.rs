mod infix;
mod literal;
mod prefix;

use crate::core::ast::Expression;

use super::Compiler;

impl Compiler {
    pub(super) fn compile_expression(&mut self, e: Expression) -> Result<(), String> {
        match e {
            Expression::Identifier(_) => todo!(),
            Expression::Integer(integer) => self.compile_integer(integer),
            Expression::String(_) => todo!(),
            Expression::Boolean(boolean) => self.compile_boolean(boolean),
            Expression::Prefix(prefix) => self.compile_prefix(prefix),
            Expression::Infix(infix) => self.compile_infix(infix),
            Expression::If(_) => todo!(),
            Expression::Function(_) => todo!(),
            Expression::Call(_) => todo!(),
        }
    }
}
