use repr::BinaryOp;

use crate::core::ast::{self, Block};

use super::{repr, BasicBlock, BasicBlockIdx, FunctionIdx, IRCtx, Triple, TripleRef, Value};

pub fn lower(program: ast::Program) -> IRCtx {
    let mut ir = IRCtx::new(program.symbols);

    lower_function(&mut ir, program.main);

    ir
}

struct FunctionLoweringCtx {
    current_bb: BasicBlockIdx,
    function: repr::Function,
}

impl FunctionLoweringCtx {
    fn add_triple(&mut self, triple: Triple) -> TripleRef {
        TripleRef {
            basic_block: self.current_bb,
            triple: self
                .function
                .basic_blocks
                .get_mut(self.current_bb)
                .expect("current basic block must exist")
                .add_triple(triple),
        }
    }
}

fn lower_function(ir_ctx: &mut IRCtx, function: ast::Function) -> FunctionIdx {
    let mut repr_function = repr::Function::new(function.name);

    // Insert entry basic block
    assert!(
        repr_function.basic_blocks.is_empty(),
        "entry basic block should be first in function"
    );
    let entry = repr_function.basic_blocks.push(BasicBlock::default());

    // Perform the lowering
    let repr_function = {
        let mut ctx = FunctionLoweringCtx {
            current_bb: entry,
            function: repr_function,
        };

        lower_block(&mut ctx, &function.body);

        ctx.function
    };

    ir_ctx.functions.push(repr_function)
}

/// Lower an AST block into the current function context.
fn lower_block(ctx: &mut FunctionLoweringCtx, block: &Block) -> Option<Value> {
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
                let value = lower_expression(ctx, value);
                ctx.add_triple(Triple::Return(value));
            }
            ast::Statement::Let(ast::LetStatement { name, value, .. }) => {
                assert!(
                    // Insert function name into scope
                    ctx.function.scope.insert(*name),
                    "cannot redeclare variable"
                );

                let value = lower_expression(ctx, value);
                ctx.add_triple(Triple::Assign(*name, value));
            }
            ast::Statement::Expression(ast::ExpressionStatement { expression, .. }) => {
                let result = lower_expression(ctx, expression);

                // Implicit return
                // TODO: Check for semi-colon
                if end {
                    return Some(result);
                }
            }
        }
    }

    // TODO: Should be unit value
    None
}

fn lower_expression(ctx: &mut FunctionLoweringCtx, expression: &ast::Expression) -> Value {
    match expression {
        ast::Expression::Infix(ast::Infix {
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
        ast::Expression::Integer(integer) => Value::Constant(integer.value),
        ast::Expression::Boolean(_) => todo!(),
        ast::Expression::Ident(ast::Ident { name, .. }) => Value::Name(*name),
        ast::Expression::Block(block) => lower_block(ctx, block).expect("block must yield value"),
        ast::Expression::If(ast::If {
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
            let success_value = lower_block(ctx, success).expect("branch to have value");

            // Lower the otherwise block, if it exists
            let (otherwise_bb, otherwise_value) = otherwise
                .as_ref()
                .map(|otherwise| {
                    let otherwise_bb = ctx.function.basic_blocks.push(BasicBlock::default());
                    let otherwise_value =
                        lower_block(ctx, otherwise).expect("else branch to have value");

                    (otherwise_bb, otherwise_value)
                })
                .expect("else branch to have value");

            // Revert back to original location
            ctx.current_bb = here;

            Value::Triple(ctx.add_triple(Triple::Switch {
                value: condition,
                default: (success_bb, success_value),
                branches: vec![(Value::Constant(0), otherwise_bb, otherwise_value)],
            }))
        }
    }
}
