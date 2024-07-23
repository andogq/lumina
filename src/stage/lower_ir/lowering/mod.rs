use crate::repr::{ast::typed as ast, ir::*};

use super::{FunctionBuilder, IRCtx};

pub fn lower(ctx: &mut impl IRCtx, program: ast::Program) {
    // Fill up the functions in the IR
    for function in program.functions {
        lower_function(ctx, function);
    }

    lower_function(ctx, program.main);
}

fn lower_function(ctx: &mut impl IRCtx, function: ast::Function) {
    // Create a new function builder, which will already be positioned at the entry point.
    let mut builder = ctx.new_builder(&function);

    // Perform the lowering
    lower_block(ctx, &mut builder, &function.body);

    // Consume the builder
    builder.build(ctx);
}

/// Lower an AST block into the current function context.
fn lower_block(
    ctx: &mut impl IRCtx,
    builder: &mut impl FunctionBuilder,
    block: &ast::Block,
) -> Value {
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
            ast::Statement::Return(ast::ReturnStatement { value, .. }) => {
                let value = lower_expression(ctx, builder, value);
                builder.add_triple(Triple::Return(value));
            }
            ast::Statement::Let(ast::LetStatement {
                binding: name,
                value,
                ..
            }) => {
                builder.register_scoped(*name, value.get_ty_info().ty);

                let value = lower_expression(ctx, builder, value);
                builder.add_triple(Triple::Assign(*name, value));
            }
            ast::Statement::Expression(ast::ExpressionStatement { expression, .. }) => {
                let result = lower_expression(ctx, builder, expression);

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

fn lower_expression(
    ctx: &mut impl IRCtx,
    builder: &mut impl FunctionBuilder,
    expression: &ast::Expression,
) -> Value {
    match expression {
        ast::Expression::Infix(ast::Infix {
            left,
            operation,
            right,
            ..
        }) => {
            let lhs = lower_expression(ctx, builder, left);
            let rhs = lower_expression(ctx, builder, right);
            let op = BinaryOp::from(operation);

            Value::Triple(builder.add_triple(Triple::BinaryOp { lhs, rhs, op }))
        }
        ast::Expression::Integer(integer) => Value::integer(integer.value),
        ast::Expression::Boolean(boolean) => Value::boolean(boolean.value),
        ast::Expression::Ident(ast::Ident { binding: name, .. }) => Value::Name(*name),
        ast::Expression::Block(block) => lower_block(ctx, builder, block),
        ast::Expression::If(ast::If {
            condition,
            success,
            otherwise,
            ..
        }) => {
            let condition = lower_expression(ctx, builder, condition);

            let original_bb = builder.current_bb();

            // Lower success block into newly created basic block
            let success_bb = builder.push_bb();
            let success_value = lower_block(ctx, builder, success);

            // Lower the otherwise block, if it exists
            let (otherwise_bb, otherwise_value) = otherwise
                .as_ref()
                .map(|otherwise| {
                    let otherwise_bb = builder.push_bb();
                    let otherwise_value = lower_block(ctx, builder, otherwise);

                    (otherwise_bb, otherwise_value)
                })
                .expect("else branch to have value");

            // Revert back to original location
            builder.goto_bb(original_bb);

            Value::Triple(builder.add_triple(Triple::Switch {
                value: condition,
                default: (success_bb, success_value),
                branches: vec![(Value::integer(0), otherwise_bb, otherwise_value)],
            }))
        }
        ast::Expression::Call(call) => {
            let idx = call.name;
            Value::Triple(builder.add_triple(Triple::Call(idx)))
        }
    }
}
