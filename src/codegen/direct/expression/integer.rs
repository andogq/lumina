use inkwell::values::IntValue;

use crate::{codegen::direct::CompilePass, core::ast::Integer};

impl Integer {
    pub fn compile<'ctx>(&self, pass: &mut CompilePass<'ctx>) -> IntValue<'ctx> {
        pass.context.i64_type().const_int(self.value as u64, true)
    }
}
