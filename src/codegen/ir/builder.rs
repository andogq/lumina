use crate::core::ast::{self, *};

use super::*;

#[derive(Default, Clone, Debug)]
pub struct Builder {
    ctx: IRContext,
    basic_block: Option<usize>,
}

impl Builder {
    pub fn consume(self) -> IRContext {
        self.ctx
    }

    /// Lower the function, returning the basic block corresponding to the entry point for the
    /// function.
    pub fn lower_function(&mut self, function: Function) -> usize {
        // Create the entry point, and set it as the current basic block
        self.new_basic_block();
        let entry = self.basic_block.expect("inside basic block");

        // Lower the body of the function, retaining the entry point.
        self.lower_block(function.body);

        // Save it in the function map
        self.ctx.functions.insert(function.name, entry);

        entry
    }

    /// Lower the provided block into the current basic block. May lead to the current basic block
    /// changing.
    fn lower_block(&mut self, block: Block) -> Option<Value> {
        let statement_count = block.statements.len();

        for (end, statement) in block
            .statements
            .into_iter()
            .enumerate()
            .map(|(i, statement)| (i == statement_count - 1, statement))
        {
            match statement {
                ast::Statement::Return(ReturnStatement { value, .. }) => {
                    let value = self.lower_expression(value);
                    self.add_triple(Triple::Return(value));
                }
                ast::Statement::Let(LetStatement { name, value, .. }) => {
                    self.ctx.symbols.push(name);
                    let value = self.lower_expression(value);
                    self.add_triple(Triple::Assign(name, value));
                }
                ast::Statement::Expression(ExpressionStatement { expression, .. }) => {
                    let result = self.lower_expression(expression);

                    // If this is the last statement in the block, return the value
                    if end {
                        return Some(result);
                    }
                }
            }
        }

        None
    }

    /// Lower an expression into the current basic block.
    fn lower_expression(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Infix(Infix {
                left,
                operation,
                right,
                ..
            }) => {
                let lhs = self.lower_expression(*left);
                let rhs = self.lower_expression(*right);
                let op = BinaryOp::from(operation);

                let triple = self.add_triple(Triple::BinaryOp { lhs, rhs, op });

                Value::Triple(triple)
            }
            Expression::Integer(integer) => Value::Constant(integer.value),
            Expression::Boolean(_) => todo!(),
            Expression::Ident(Ident { name, .. }) => Value::Name(name),
            Expression::Block(block) => self.lower_block(block).expect("block should yield value"),
            Expression::If(If {
                condition,
                success,
                otherwise,
                ..
            }) => {
                let condition = self.lower_expression(*condition);

                let here = self.basic_block.expect("to be in basic block");

                // Lower success block into newly created basic block
                let success_bb = self.new_basic_block();
                let success_value = self.lower_block(success).expect("branch to have value");

                // Lower the otherwise block, if it exists
                let (otherwise_bb, otherwise_value) = otherwise
                    .map(|otherwise| {
                        let otherwise_bb = self.new_basic_block();
                        let otherwise_value = self
                            .lower_block(otherwise)
                            .expect("else branch to have value");

                        (otherwise_bb, otherwise_value)
                    })
                    .expect("else branch to have value");

                // Revert back to original location
                self.basic_block = Some(here);

                Value::Triple(self.add_triple(Triple::Switch {
                    value: condition,
                    default: (success_bb, success_value),
                    branches: vec![(Value::Constant(0), otherwise_bb, otherwise_value)],
                }))
            }
        }
    }

    /// Add a triple to the current basic block.
    fn add_triple(&mut self, triple: Triple) -> TripleRef {
        let basic_block_id = self.basic_block.expect("in active basic block");
        let basic_block = self
            .ctx
            .basic_blocks
            .get_mut(basic_block_id)
            .expect("basic block to exist");

        let triple_id = basic_block.add_triple(triple);

        TripleRef::new(basic_block_id, triple_id)
    }

    /// Create a new basic block, and switch to it.
    fn new_basic_block(&mut self) -> usize {
        let id = self.ctx.new_basic_block();
        self.basic_block = Some(id);
        id
    }
}
