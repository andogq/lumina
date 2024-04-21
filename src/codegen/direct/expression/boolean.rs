use inkwell::values::IntValue;

use crate::{codegen::direct::CompilePass, core::ast::Boolean};

impl Boolean {
    pub fn compile<'ctx>(&self, pass: &mut CompilePass<'ctx>) -> IntValue<'ctx> {
        if self.value {
            pass.context.bool_type().const_all_ones()
        } else {
            pass.context.bool_type().const_zero()
        }
    }
}
