use std::collections::HashMap;

use inkwell::values::{IntValue, PointerValue};

use crate::{
    codegen::CompilePass,
    core::{ast::Ident, symbol::Symbol},
};

impl Ident {
    pub fn compile<'ctx>(
        &self,
        pass: &mut CompilePass<'ctx>,
        scope: &mut HashMap<Symbol, PointerValue<'ctx>>,
    ) -> IntValue<'ctx> {
        pass.builder
            .build_load(
                pass.context.i64_type(),
                scope.get(&self.name).unwrap().clone(),
                &pass.symbol_map.name(self.name).unwrap(),
            )
            .unwrap()
            .into_int_value()
    }
}
