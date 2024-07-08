use repr::BinaryOp;

use crate::core::ty::ast::*;

use super::{repr, BasicBlock, BasicBlockIdx, FunctionIdx, IRCtx, Triple, TripleRef, Value};

pub fn lower(program: Program) -> IRCtx {
    let mut ir = IRCtx::new(program.symbols);

    // Fill up the functions in the IR
    for function in program.functions {
        lower_function(&mut ir, function);
    }

    lower_function(&mut ir, program.main);

    ir
}

struct FunctionLoweringCtx<'ctx> {
    ir_ctx: &'ctx mut IRCtx,
    current_bb: BasicBlockIdx,
    function: repr::Function,
}

impl FunctionLoweringCtx<'_> {
    fn add_triple(&mut self, triple: Triple) -> TripleRef {
        TripleRef {
            basic_block: self.current_bb,
            triple: self
                .function
                .basic_blocks
                .get_mut(self.current_bb)
                .expect("current basic block must exist")
                .triples
                .push(triple),
        }
    }
}

fn lower_function(ir_ctx: &mut IRCtx, function: Function) -> FunctionIdx {
    let mut repr_function = repr::Function::new(&function);

    // Insert entry basic block
    assert!(
        repr_function.basic_blocks.is_empty(),
        "entry basic block should be first in function"
    );
    let entry = repr_function.basic_blocks.push(BasicBlock::default());

    // Perform the lowering
    let repr_function = {
        let mut ctx = FunctionLoweringCtx {
            ir_ctx,
            current_bb: entry,
            function: repr_function,
        };

        lower_block(&mut ctx, &function.body);

        ctx.function
    };

    ir_ctx.functions.push(repr_function)
}

/// Lower an AST block into the current function context.
fn lower_block(ctx: &mut FunctionLoweringCtx, block: &Block) -> Value {
    assert!(
        !block.statements.is_empty(),
        "block must have statements within it"
    );

    // Determine the index of the last statement
    let last_statement = block.statements.len() - 1;

    for (end, statement) in block
        .statements
        .iter()
        .enumerate()
        .map(|(i, statement)| (i == last_statement, statement))
    {
        match statement {
            Statement::Return(ReturnStatement { value, .. }) => {
                let value = lower_expression(ctx, value);
                ctx.add_triple(Triple::Return(value));
            }
            Statement::Let(LetStatement { name, value, .. }) => {
                assert!(
                    // Insert function name into scope
                    ctx.function.scope.insert(*name),
                    "cannot redeclare variable"
                );

                let value = lower_expression(ctx, value);
                ctx.add_triple(Triple::Assign(*name, value));
            }
            Statement::Expression(ExpressionStatement { expression, .. }) => {
                let result = lower_expression(ctx, expression);

                // Implicit return
                // TODO: Check for semi-colon
                if end {
                    return result;
                }
            }
        }
    }

    // TODO: Should be unit value
    Value::Unit
}

fn lower_expression(ctx: &mut FunctionLoweringCtx, expression: &Expression) -> Value {
    match expression {
        Expression::Infix(Infix {
            left,
            operation,
            right,
            ..
        }) => {
            let lhs = lower_expression(ctx, left);
            let rhs = lower_expression(ctx, right);
            let op = BinaryOp::from(operation);

            Value::Triple(ctx.add_triple(Triple::BinaryOp { lhs, rhs, op }))
        }
        Expression::Integer(integer) => Value::integer(integer.value),
        Expression::Boolean(boolean) => Value::boolean(boolean.value),
        Expression::Ident(Ident { name, .. }) => Value::Name(*name),
        Expression::Block(block) => lower_block(ctx, block),
        Expression::If(If {
            condition,
            success,
            otherwise,
            ..
        }) => {
            let condition = lower_expression(ctx, condition);

            let here = ctx.current_bb;

            // Lower success block into newly created basic block
            let success_bb = ctx.function.basic_blocks.push(BasicBlock::default());
            ctx.current_bb = success_bb;
            let success_value = lower_block(ctx, success);

            // Lower the otherwise block, if it exists
            let (otherwise_bb, otherwise_value) = otherwise
                .as_ref()
                .map(|otherwise| {
                    let otherwise_bb = ctx.function.basic_blocks.push(BasicBlock::default());
                    ctx.current_bb = otherwise_bb;
                    let otherwise_value = lower_block(ctx, otherwise);

                    (otherwise_bb, otherwise_value)
                })
                .expect("else branch to have value");

            // Revert back to original location
            ctx.current_bb = here;

            Value::Triple(ctx.add_triple(Triple::Switch {
                value: condition,
                default: (success_bb, success_value),
                branches: vec![(Value::integer(0), otherwise_bb, otherwise_value)],
            }))
        }
        Expression::Call(call) => {
            let idx = ctx.ir_ctx.function_for_symbol(call.name).unwrap();
            Value::Triple(ctx.add_triple(Triple::Call(idx)))
        }
    }
}
