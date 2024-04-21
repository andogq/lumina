use std::collections::HashMap;

use inkwell::{
    values::{IntValue, PointerValue},
    IntPredicate,
};

use crate::{
    codegen::direct::CompilePass,
    core::{ast::If, symbol::Symbol},
};

impl If {
    pub fn compile<'ctx>(
        &self,
        pass: &mut CompilePass<'ctx>,
        scope: &mut HashMap<Symbol, PointerValue<'ctx>>,
    ) -> IntValue<'ctx> {
        let condition = self.condition.compile(pass, scope);

        let result = pass
            .builder
            .build_int_compare(
                IntPredicate::NE,
                condition,
                pass.context.bool_type().const_zero(),
                "condition",
            )
            .unwrap();

        let function = pass
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();
        let success_block = pass.context.append_basic_block(function, "success");
        let otherwise_block = pass.context.append_basic_block(function, "otherwise");
        let merge_block = pass.context.append_basic_block(function, "if cont");

        // Create the conditional branch instruction
        pass.builder
            .build_conditional_branch(result, success_block, otherwise_block)
            .unwrap();

        // Compile success branch
        pass.builder.position_at_end(success_block);
        let success = self.success.compile(pass, scope);

        pass.builder
            .build_unconditional_branch(merge_block)
            .unwrap();

        // Update the location of the success block, since the implementation may end in a
        // different basic block
        let success_end_block = pass.builder.get_insert_block().unwrap();

        let otherwise = if let Some(otherwise) = &self.otherwise {
            // Compile otherwise branch
            pass.builder.position_at_end(otherwise_block);
            let otherwise = otherwise.compile(pass, scope);

            pass.builder
                .build_unconditional_branch(merge_block)
                .unwrap();

            otherwise
        } else {
            None
        };
        let otherwise_end_block = pass.builder.get_insert_block().unwrap();

        // Insert the Phi instruction
        pass.builder.position_at_end(merge_block);
        let phi = pass
            .builder
            .build_phi(pass.context.i64_type(), "iftmp")
            .unwrap();
        phi.add_incoming(&[
            (&success.unwrap(), success_end_block),
            (&otherwise.unwrap(), otherwise_end_block),
        ]);

        phi.as_basic_value().into_int_value()
    }
}
