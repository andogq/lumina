use std::collections::HashMap;

use inkwell::values::{IntValue, PointerValue};

use crate::core::{ast::Statement, symbol::Symbol};

use super::CompilePass;

impl Statement {
    pub fn compile<'ctx>(
        &self,
        pass: &mut CompilePass<'ctx>,
        scope: &mut HashMap<Symbol, PointerValue<'ctx>>,
    ) -> Option<IntValue<'ctx>> {
        match self {
            Statement::Return(s) => {
                let value = s.value.compile(pass, scope);
                pass.builder.build_return(Some(&value)).unwrap();
                None
            }
            Statement::Expression(s) => Some(s.expression.compile(pass, scope)),
            Statement::Let(s) => {
                // Compile value of let statement
                let value = s.value.compile(pass, scope);

                // Create a place for the variable to be stored
                let entry = pass
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap()
                    .get_first_basic_block()
                    .unwrap();

                // Create a new builder to not change the position of the current builder
                let stack_builder = pass.context.create_builder();

                // Position builder to be at the start of the entry block
                match entry.get_first_instruction() {
                    Some(instr) => stack_builder.position_before(&instr),
                    None => stack_builder.position_at_end(entry),
                };

                // Stack address that this variable will be stored at
                let addr = pass
                    .builder
                    .build_alloca(
                        // TODO: This allocation should depend on the type of the variable
                        pass.context.i64_type(),
                        pass.symbols.resolve(s.name).unwrap(),
                    )
                    .unwrap();

                // Move statement value onto stack
                pass.builder.build_store(addr, value).unwrap();

                // Add address to the symbol table
                scope.insert(s.name, addr);

                None
            }
        }
    }
}
