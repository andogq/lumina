use std::collections::HashMap;

use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, PointerValue},
};

use crate::{
    codegen::ir::{Statement, Terminator},
    util::index::IndexVec,
};

use super::ir::{
    value::{Local, RValue},
    BasicBlockData, Function, RETURN_LOCAL,
};

type Locals<'ctx> = HashMap<Local, PointerValue<'ctx>>;

pub fn compile<'ctx>(
    ctx: &'ctx Context,
    module: &Module<'ctx>,
    function: Function,
    bbs: IndexVec<BasicBlockData>,
) -> FunctionValue<'ctx> {
    let builder = ctx.create_builder();

    // Create the prototype of the function
    // TODO: Currently only accepts functions that return an integer
    let fn_type = ctx.i64_type().fn_type(&[], false);
    let fn_value = module.add_function(&function.name.to_string(), fn_type, None);

    // Create the entry basic block
    let entry = ctx.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry);

    // Prepare locals for the function body
    let mut locals = HashMap::new();
    locals.insert(RETURN_LOCAL, {
        builder
            .build_alloca(ctx.i64_type(), "return value")
            .unwrap()
    });

    compile_basic_block(
        ctx,
        entry,
        locals,
        bbs.get(function.entry).unwrap().to_owned(),
    );

    fn_value
}

fn compile_basic_block(
    ctx: &Context,
    target: inkwell::basic_block::BasicBlock,
    locals: Locals,
    basic_block: BasicBlockData,
) {
    let builder = ctx.create_builder();
    builder.position_at_end(target);

    for statement in basic_block.statements {
        match statement {
            Statement::Assign(local, value) => {
                let ptr = locals.get(&local).unwrap().to_owned();

                let value = match value {
                    RValue::Scalar(s) => {
                        // TODO: Properly handle arbitrary precision
                        ctx.i64_type().const_int(s.data, false)
                    }
                };

                builder.build_store(ptr, value).unwrap();
            }
        }
    }

    match basic_block.terminator {
        Terminator::Return => {
            builder
                .build_return(Some({
                    // TODO: Assumes that there is a return value
                    let ptr = locals.get(&RETURN_LOCAL).unwrap();

                    &builder
                        .build_load(ctx.i64_type(), *ptr, "load return")
                        .unwrap()
                }))
                .unwrap();
        }
    }
}
