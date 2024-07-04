use std::collections::HashMap;

use inkwell::values::{IntValue, PointerValue};

use crate::{
    codegen::direct::CompilePass,
    core::{
        ast::{Infix, InfixOperation},
        symbol::Symbol,
    },
};

impl Infix {
    pub fn compile<'ctx>(
        &self,
        pass: &mut CompilePass<'ctx>,
        scope: &mut HashMap<Symbol, PointerValue<'ctx>>,
    ) -> IntValue<'ctx> {
        let left = self.left.compile(pass, scope);
        let right = self.right.compile(pass, scope);

        match self.operation {
            InfixOperation::Plus(_) => pass.builder.build_int_add(left, right, "temp_add").unwrap(),
            InfixOperation::Eq(_) => unimplemented!(),
            InfixOperation::NotEq(_) => unimplemented!(),
        }
    }
}
