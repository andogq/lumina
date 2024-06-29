use std::collections::HashMap;

use crate::core::{
    ast::{
        self, Block, Expression, ExpressionStatement, Function, Ident, Infix, LetStatement,
        ReturnStatement,
    },
    symbol::Symbol,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
}

impl From<ast::InfixOperation> for BinaryOp {
    fn from(value: ast::InfixOperation) -> Self {
        match value {
            ast::InfixOperation::Plus(_) => Self::Add,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnaryOp {
    Minus,
    Not,
}

/// A reference to a specific triple.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TripleRef {
    pub basic_block: usize,
    pub triple: usize,
}

impl TripleRef {
    pub fn new(basic_block: usize, triple: usize) -> Self {
        Self {
            basic_block,
            triple,
        }
    }
}

/// Corresponds to the 'address' portion of a three-address code. Intended to transparently
/// represent any possible source of a value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Value {
    /// Value derived from a name in the source code.
    Name(Symbol),
    /// Constant value, potentially inserted from the compiler or originating from the source code.
    Constant(i64),
    /// Temporary value representing the result of some triple.
    Triple(TripleRef),
}

/// Each possible operation of the IR. The results of these operations (if applicable) can be
/// referenced using the ID of the triple.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Triple {
    /// Standard binary operation.
    BinaryOp {
        lhs: Value,
        rhs: Value,
        op: BinaryOp,
    },
    /// Standard unary operation.
    UnaryOp { rhs: Value, op: UnaryOp },
    /// Copy the provided value.
    Copy(Value),
    /// Jump to the corresponding basic block.
    Jump(usize),
    /// Jump to the corresponding basic block if the value is not zero.
    CondJump(Value, usize),
    /// Call the corresponding function.
    Call(Symbol),
    /// Return with the provided value.
    Return(Value),
    /// Assign some symbol to some value.
    Assign(Symbol, Value),
}

#[derive(Default, Clone, Debug)]
pub struct BasicBlock {
    pub triples: Vec<Triple>,
}

impl BasicBlock {
    pub fn add_triple(&mut self, triple: Triple) -> usize {
        let id = self.triples.len();
        self.triples.push(triple);

        id
    }
}

#[derive(Default, Clone, Debug)]
pub struct IRContext {
    /// Map of function symbol to the basic block entry point
    pub functions: HashMap<Symbol, usize>,
    pub basic_blocks: Vec<BasicBlock>,
    pub symbols: Vec<Symbol>,
}

impl IRContext {
    /// Create a new basic block, returning a reference to it.
    fn new_basic_block(&mut self) -> usize {
        let id = self.basic_blocks.len();
        self.basic_blocks.push(BasicBlock::default());
        id
    }
}

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
    fn lower_block(&mut self, block: Block) {
        for statement in block.statements {
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
                    // TODO: Likely will need to somehow return from this block with a value
                    self.lower_expression(expression);
                }
            }
        }
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
            Expression::Block(_) => todo!(),
            Expression::If(_) => todo!(),
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
    fn new_basic_block(&mut self) {
        let id = self.ctx.new_basic_block();
        self.basic_block = Some(id);
    }
}
