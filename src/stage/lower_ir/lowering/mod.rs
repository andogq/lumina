use crate::repr::{ast::typed as ast, ir::*, ty::Ty};

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

    // Load all the parameters
    function
        .parameters
        .iter()
        .enumerate()
        .for_each(|(i, (binding, _ty))| {
            // Copy each parameter into its binding
            builder.add_triple(Triple::Assign(*binding, Value::Parameter(i)));
        });

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
                let value = lower_expression(ctx, builder, value).unwrap();
                builder.add_triple(Triple::Return(value));
            }
            ast::Statement::Let(ast::LetStatement {
                binding: name,
                value,
                ..
            }) => {
                builder.register_scoped(*name, value.get_ty_info().ty);

                let value = lower_expression(ctx, builder, value).unwrap();
                builder.add_triple(Triple::Assign(*name, value));
            }
            ast::Statement::Expression(ast::ExpressionStatement {
                expression,
                ty_info,
                ..
            }) => {
                let result = lower_expression(ctx, builder, expression);

                // Implicit return
                if end && !matches!(ty_info.ty, Ty::Never) {
                    // WARN: If the type is `never`, what should be returned?
                    return result.unwrap();
                }
            }
        }
    }

    Value::Unit
}

fn lower_expression(
    ctx: &mut impl IRCtx,
    builder: &mut impl FunctionBuilder,
    expression: &ast::Expression,
) -> Option<Value> {
    match expression {
        ast::Expression::Infix(ast::Infix {
            left,
            operation,
            right,
            ..
        }) => {
            let lhs = lower_expression(ctx, builder, left).unwrap();
            let rhs = lower_expression(ctx, builder, right).unwrap();
            let op = BinaryOp::from(operation);

            Some(Value::Triple(builder.add_triple(Triple::BinaryOp {
                lhs,
                rhs,
                op,
            })))
        }
        ast::Expression::Integer(integer) => Some(Value::integer(integer.value)),
        ast::Expression::Boolean(boolean) => Some(Value::boolean(boolean.value)),
        ast::Expression::Ident(ast::Ident { binding: name, .. }) => Some(Value::Name(*name)),
        ast::Expression::Block(block) => Some(lower_block(ctx, builder, block)),
        ast::Expression::If(ast::If {
            condition,
            success,
            otherwise,
            ty_info,
            ..
        }) => {
            let condition = lower_expression(ctx, builder, condition).unwrap();

            let original_bb = builder.current_bb();

            // Prepare a basic block to merge back in to
            let merge_bb = if matches!(ty_info.ty, Ty::Never) && otherwise.is_some() {
                None
            } else {
                Some(builder.push_bb())
            };

            // Lower success block into newly created basic block
            let success_bb = builder.push_bb();
            let success_value = lower_block(ctx, builder, success);

            if let (Some(merge_bb), true) = (merge_bb, !matches!(success.ty_info.ty, Ty::Never)) {
                // Ensure the branch returns to the merge basic block
                builder.add_triple(Triple::Jump(merge_bb));
            }

            let mut merge_values = vec![(success_value, success_bb)];

            // Lower the otherwise block, if it exists
            let branches = [otherwise
                .as_ref()
                .map(|otherwise| {
                    let otherwise_bb = builder.push_bb();
                    let otherwise_value = lower_block(ctx, builder, otherwise);

                    if let Some(merge_bb) = merge_bb {
                        // Ensure the branch returns to the merge basic block
                        builder.add_triple(Triple::Jump(merge_bb));
                    }

                    merge_values.push((otherwise_value, otherwise_bb));

                    otherwise_bb
                })
                .or(merge_bb)
                .map(|bb| (Value::boolean(false), bb))]
            .into_iter()
            .flatten()
            .collect();

            // Revert back to original location
            builder.goto_bb(original_bb);

            builder.add_triple(Triple::Switch {
                value: condition,
                default: success_bb,
                branches,
            });

            // Continue inserting triples from the merged location
            if let Some(merge_bb) = merge_bb {
                builder.goto_bb(merge_bb);
            }

            match ty_info.ty {
                Ty::Unit => Some(Value::Unit),
                Ty::Never => None,
                _ => {
                    // Only insert the phi node if the if statement is an expression
                    Some(Value::Triple(builder.add_triple(Triple::Phi(merge_values))))
                }
            }
        }
        ast::Expression::Call(call) => {
            let idx = call.name;
            let params = call
                .args
                .iter()
                .map(|e| lower_expression(ctx, builder, e).unwrap())
                .collect();
            Some(Value::Triple(builder.add_triple(Triple::Call(idx, params))))
        }
    }
}
