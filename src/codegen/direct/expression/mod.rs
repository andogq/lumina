mod block;
mod boolean;
mod ident;
mod if_else;
mod infix;
mod integer;

use std::collections::HashMap;

use inkwell::values::{IntValue, PointerValue};

use crate::core::{ast::Expression, symbol::Symbol};

use super::CompilePass;

impl Expression {
    pub fn compile<'ctx>(
        &self,
        pass: &mut CompilePass<'ctx>,
        scope: &mut HashMap<Symbol, PointerValue<'ctx>>,
    ) -> IntValue<'ctx> {
        match self {
            Expression::Infix(infix) => infix.compile(pass, scope),
            Expression::Integer(integer) => integer.compile(pass),
            Expression::Ident(ident) => ident.compile(pass, scope),
            Expression::Boolean(boolean) => boolean.compile(pass),
            Expression::Block(block) => block
                .compile(pass, scope)
                .expect("block must evaluate to a value"),
            Expression::If(if_else) => if_else.compile(pass, scope),
        }
    }
}
