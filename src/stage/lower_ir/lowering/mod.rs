use index_vec::IndexVec;

use crate::{
    compiler::Compiler,
    repr::{
        ast::typed as ast,
        identifier::{FunctionIdx, ScopedBinding},
        ir::{self, *},
        ty::Ty,
    },
    stage::type_check::FunctionSignature,
};

pub fn lower(compiler: &mut Compiler, program: ast::Program) -> Vec<Function> {
    [lower_function(compiler, program.main)]
        .into_iter()
        .chain(
            program
                .functions
                .into_iter()
                .map(|function| lower_function(compiler, function)),
        )
        .collect()
}

#[derive(Debug, Default)]
struct BasicBlockBuilder {
    triples: IndexVec<ir::TripleIdx, Triple>,
    terminator: Option<Terminator>,
}

pub struct FunctionBuilder {
    idx: FunctionIdx,
    signature: FunctionSignature,

    basic_blocks: IndexVec<ir::BasicBlockIdx, BasicBlockBuilder>,
    current_basic_block: ir::BasicBlockIdx,

    /// Tracks the starting and ending basic block for any loops, so they can be jumped back to it
    loop_stack: Vec<(ir::BasicBlockIdx, ir::BasicBlockIdx)>,

    scope: Vec<(ScopedBinding, Ty)>,
}

impl FunctionBuilder {
    pub fn new(function: &ast::Function) -> Self {
        let mut basic_blocks = IndexVec::new();
        let current_basic_block = basic_blocks.push(BasicBlockBuilder::default());

        Self {
            idx: function.name,
            signature: FunctionSignature::from(function),
            basic_blocks,
            current_basic_block,
            loop_stack: Vec::new(),
            scope: function.parameters.to_vec(),
        }
    }

    pub fn register_scoped(&mut self, ident: ScopedBinding, ty: Ty) {
        self.scope.push((ident, ty));
    }

    pub fn add_triple(&mut self, triple: ir::Triple) -> ir::TripleRef {
        ir::TripleRef {
            basic_block: self.current_basic_block,
            triple: self.basic_blocks[self.current_basic_block]
                .triples
                .push(triple),
        }
    }

    pub fn set_terminator(&mut self, terminator: ir::Terminator) {
        let bb = &mut self.basic_blocks[self.current_basic_block];

        assert!(
            bb.terminator.is_none(),
            "cannot set terminator if it's already been set"
        );

        bb.terminator = Some(terminator);
    }

    pub fn current_bb(&self) -> ir::BasicBlockIdx {
        self.current_basic_block
    }

    pub fn goto_bb(&mut self, bb: ir::BasicBlockIdx) {
        assert!(
            bb < self.basic_blocks.len_idx(),
            "can only goto basic block if it exists"
        );
        self.current_basic_block = bb;
    }

    pub fn push_bb(&mut self) -> ir::BasicBlockIdx {
        let idx = self.basic_blocks.push(BasicBlockBuilder::default());

        self.current_basic_block = idx;

        idx
    }

    pub fn build(self) -> Function {
        Function {
            identifier: self.idx,
            signature: self.signature,
            basic_blocks: self
                .basic_blocks
                .into_iter()
                .map(|builder| ir::BasicBlock {
                    triples: builder.triples,
                    terminator: builder.terminator.expect("terminator must be set"),
                })
                .collect(),
            scope: self.scope.into_iter().map(|(symbol, _)| symbol).collect(),
        }
    }
}

fn lower_function(compiler: &mut Compiler, function: ast::Function) -> Function {
    // Create a new function builder, which will already be positioned at the entry point.
    let mut builder = FunctionBuilder::new(&function);

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
    let value = lower_block(compiler, &mut builder, &function.body);

    // If implicit return, add in a return statement
    if !matches!(value, Value::Unit) {
        builder.set_terminator(Terminator::Return(value));
    }

    // Consume the builder
    builder.build()
}

/// Lower an AST block into the current function context.
fn lower_block(
    compiler: &mut Compiler,
    builder: &mut FunctionBuilder,
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
                let value = lower_expression(compiler, builder, value).unwrap();
                builder.set_terminator(Terminator::Return(value));
            }
            ast::Statement::Let(ast::LetStatement {
                binding: name,
                value,
                ..
            }) => {
                builder.register_scoped(*name, value.get_ty_info().ty.clone());

                let value = lower_expression(compiler, builder, value).unwrap();
                builder.add_triple(Triple::Assign(*name, value));
            }
            ast::Statement::Break(ast::BreakStatement { .. }) => {
                assert!(
                    !builder.loop_stack.is_empty(),
                    "can only break within a loop"
                );

                let (_, loop_end) = builder.loop_stack.last().unwrap();
                builder.set_terminator(Terminator::Jump(*loop_end));
            }
            ast::Statement::Continue(ast::ContinueStatement { .. }) => {
                assert!(
                    !builder.loop_stack.is_empty(),
                    "can only continue within a loop"
                );

                let (loop_start, _) = builder.loop_stack.last().unwrap();
                builder.set_terminator(Terminator::Jump(*loop_start));
            }
            ast::Statement::ExpressionStatement(ast::ExpressionStatement {
                expression,
                ty_info,
                ..
            }) => {
                let result = lower_expression(compiler, builder, expression);

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
    compiler: &mut Compiler,
    builder: &mut FunctionBuilder,
    expression: &ast::Expression,
) -> Option<Value> {
    match expression {
        ast::Expression::Infix(ast::Infix {
            left,
            operation,
            right,
            ..
        }) => {
            let lhs = lower_expression(compiler, builder, left).unwrap();
            let rhs = lower_expression(compiler, builder, right).unwrap();
            let op = BinaryOp::from(operation);

            Some(Value::Triple(builder.add_triple(Triple::BinaryOp {
                lhs,
                rhs,
                op,
            })))
        }
        ast::Expression::Integer(integer) => Some(Value::integer(integer.value)),
        ast::Expression::Boolean(boolean) => Some(Value::boolean(boolean.value)),
        ast::Expression::Ident(ast::Ident { binding, .. }) => {
            Some(Value::Triple(builder.add_triple(Triple::Load(*binding))))
        }
        ast::Expression::Block(block) => Some(lower_block(compiler, builder, block)),
        ast::Expression::If(ast::If {
            condition,
            success,
            otherwise,
            ty_info,
            ..
        }) => {
            let condition = lower_expression(compiler, builder, condition).unwrap();

            let original_bb = builder.current_bb();

            // Prepare a basic block to merge back in to
            let merge_bb = if matches!(ty_info.ty, Ty::Never) && otherwise.is_some() {
                None
            } else {
                Some(builder.push_bb())
            };

            // Lower success block into newly created basic block
            let success_bb = builder.push_bb();
            let success_value = lower_block(compiler, builder, success);

            if let (Some(merge_bb), true) = (merge_bb, !matches!(success.ty_info.ty, Ty::Never)) {
                // Ensure the branch returns to the merge basic block
                builder.set_terminator(Terminator::Jump(merge_bb));
            }

            let mut merge_values = vec![(success_value, success_bb)];

            // Lower the otherwise block, if it exists
            let branches = [otherwise
                .as_ref()
                .map(|otherwise| {
                    let otherwise_bb = builder.push_bb();
                    let otherwise_value = lower_block(compiler, builder, otherwise);

                    if let Some(merge_bb) = merge_bb {
                        // Ensure the branch returns to the merge basic block
                        builder.set_terminator(Terminator::Jump(merge_bb));
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

            builder.set_terminator(Terminator::Switch {
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
        ast::Expression::Loop(e_loop) => {
            let prev = builder.current_bb();

            // Create a new basic block
            let loop_start = builder.push_bb();

            // Jump from the previous block to the loop start
            builder.goto_bb(prev);
            builder.set_terminator(Terminator::Jump(loop_start));

            // Prepare an ending basic block
            let loop_end = builder.push_bb();

            // Save the start and end locations (keeping track of how many previous loops there are)
            builder.loop_stack.push((loop_start, loop_end));
            let loop_count = builder.loop_stack.len();

            // Lower the loop body
            builder.goto_bb(loop_start);
            lower_block(compiler, builder, &e_loop.body);

            // HACK: Should there be a better way of determining if the loop will never end (check for never?)
            if builder.basic_blocks[builder.current_bb()]
                .terminator
                .is_none()
            {
                // Jump from the loop end back to the start
                builder.set_terminator(Terminator::Jump(loop_start));
            }

            // Remove this loop from the stack
            assert_eq!(
                builder.loop_stack.len(),
                loop_count,
                "must be at same position in loop stack"
            );
            builder.loop_stack.pop();

            // Continue on from where the loop ends
            builder.goto_bb(loop_end);

            Some(Value::Unit)
        }
        ast::Expression::Call(call) => {
            let idx = call.name;
            let params = call
                .args
                .iter()
                .map(|e| lower_expression(compiler, builder, e).unwrap())
                .collect();
            Some(Value::Triple(builder.add_triple(Triple::Call(idx, params))))
        }
        ast::Expression::Assign(assign) => {
            let value = lower_expression(compiler, builder, &assign.value).unwrap();
            builder.add_triple(Triple::Assign(assign.binding, value));

            Some(Value::Unit)
        }
        ast::Expression::Cast(ast::Cast { value, .. }) => {
            // Directly lower the inner expression, cast is only for the compiler
            lower_expression(compiler, builder, value)
        }
        ast::Expression::Index(ast::Index { value, index, .. }) => {
            let index = lower_expression(compiler, builder, index)?;

            Some(Value::Triple(builder.add_triple(Triple::Index {
                value: *value,
                index,
            })))
        }
        ast::Expression::Array(ast::Array { init, .. }) => {
            // Allocate the memory
            let ptr = builder.add_triple(Triple::AllocArray(init.len() as u32));

            // Initialise each of the items into the memory
            init.iter().enumerate().for_each(|(i, expression)| {
                let value = lower_expression(compiler, builder, expression).unwrap();

                builder.add_triple(Triple::SetIndex {
                    array_ptr: ptr,
                    index: Value::integer(i as i64),
                    value,
                });
            });

            Some(Value::Pointer(ptr))
        }
    }
}
