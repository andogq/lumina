use std::collections::HashMap;

use inkwell::values::{IntValue, PointerValue};

use crate::{
    codegen::direct::CompilePass,
    core::{ast::Block, symbol::Symbol},
};

impl Block {
    pub fn compile<'ctx>(
        &self,
        pass: &mut CompilePass<'ctx>,
        scope: &mut HashMap<Symbol, PointerValue<'ctx>>,
    ) -> Option<IntValue<'ctx>> {
        let mut value = None;

        for statement in &self.statements {
            value = statement.compile(pass, scope);
        }

        value
    }
}
