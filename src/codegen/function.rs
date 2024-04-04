use std::collections::HashMap;

use inkwell::values::FunctionValue;

use crate::core::ast::Function;

use super::CompilePass;

impl Function {
    pub fn compile<'ctx>(&self, pass: &mut CompilePass<'ctx>) -> FunctionValue<'ctx> {
        // Create a prototype
        let fn_type = pass.context.i64_type().fn_type(&[], false);
        let fn_value =
            pass.module
                .add_function(&pass.symbol_map.name(self.name).unwrap(), fn_type, None);

        // Create the entry point and position the builder
        let entry = pass.context.append_basic_block(fn_value, "entry");
        pass.builder.position_at_end(entry);

        let mut scope = HashMap::new();

        // Compile the body
        self.body.compile(pass, &mut scope);

        // Verify and optimise the function
        fn_value.verify(true);

        fn_value
    }
}
