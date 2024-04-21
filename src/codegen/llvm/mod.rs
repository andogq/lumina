use inkwell::context::Context;

use crate::codegen::ir::Terminator;

use super::ir::{BasicBlockData, Function};

pub fn compile(ctx: &mut Context, function: Function) {
    // TODO: Resolve the actual name of the function
    let module = ctx.create_module(&function.name.to_string());
    let builder = ctx.create_builder();

    // Create the prototype of the function
    // TODO: Currently only accepts functions that return an integer
    let fn_type = ctx.i64_type().fn_type(&[], false);
    let fn_value = module.add_function(&function.name.to_string(), fn_type, None);

    // Create the entry basic block
    let entry = ctx.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry);
}

fn compile_basic_block(
    ctx: &mut Context,
    target: inkwell::basic_block::BasicBlock,
    basic_block: BasicBlockData,
) {
    let builder = ctx.create_builder();
    builder.position_at_end(target);

    // TODO: Compile statements
    assert!(basic_block.statements.is_empty());

    match basic_block.terminator {
        Terminator::Return => {
            builder.build_return(todo!()).unwrap();
        }
    }
}
