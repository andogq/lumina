mod index;

use std::{
    cell::RefCell,
    fmt::Debug,
    marker::PhantomData,
    num::NonZeroU8,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use self::index::{Index, IndexVec};

use super::ast;

#[derive(Clone)]
struct LocalDecl;
type Local = Index<LocalDecl>;
const RETURN_LOCAL: Local = Index::<LocalDecl>(0, PhantomData);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scalar {
    /// The value of this scalar.
    data: u64,

    /// Number of bytes the scalar value takes up.
    size: NonZeroU8,
}

impl PartialEq<&Scalar> for Scalar {
    fn eq(&self, other: &&Scalar) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<Scalar> for &Scalar {
    fn eq(&self, other: &Scalar) -> bool {
        (*self).eq(other)
    }
}

impl Scalar {
    pub fn int(value: i64) -> Self {
        Self {
            data: value as u64,
            size: NonZeroU8::new(8).unwrap(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum RValue {
    Scalar(Scalar),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Statement {
    Assign(Local, RValue),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Terminator {
    /// Return from the function.
    Return,
}

struct BasicBlockData {
    statements: Vec<Statement>,
    terminator: Terminator,
}

type BasicBlock = Index<BasicBlockData>;

struct BasicBlockBuilder {
    ctx: Context,
    statements: Vec<Statement>,
}

impl BasicBlockBuilder {
    pub fn statement(mut self, statement: Statement) -> Self {
        self.statements.push(statement);
        self
    }

    pub fn t_return(self) -> BasicBlock {
        self.ctx.0.borrow_mut().basic_blocks.push(BasicBlockData {
            statements: self.statements,
            terminator: Terminator::Return,
        })
    }
}

/// The locals that are in scope for the given function
struct FunctionScope {
    /// The local corresponding with the return location
    l_return: Local,

    /// All of the locals within the scope
    locals: IndexVec<LocalDecl>,
}
impl Deref for FunctionScope {
    type Target = IndexVec<LocalDecl>;

    fn deref(&self) -> &Self::Target {
        &self.locals
    }
}
impl DerefMut for FunctionScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.locals
    }
}

#[derive(Default)]
struct ContextInner {
    basic_blocks: IndexVec<BasicBlockData>,
}

#[derive(Clone)]
struct Context(Rc<RefCell<ContextInner>>);

impl Context {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Default::default())))
    }

    fn basic_block(&self) -> BasicBlockBuilder {
        BasicBlockBuilder {
            ctx: self.clone(),
            statements: Vec::new(),
        }
    }

    pub fn lower_function(&self, node: ast::Function) -> Function {
        let entry = self.basic_block();

        for s in node.body.statements {
            match s {
                ast::Statement::Return(s) => {
                    return Function {
                        entry: self
                            // Evaluate the expression into the return local
                            .lower_expression(entry, s.value, RETURN_LOCAL)
                            // Trigger the return terminator
                            .t_return(),
                    };
                }
                ast::Statement::Let(_) => todo!(),
                ast::Statement::Expression(_) => todo!(),
            }
        }

        panic!("function body had zero statements")
    }

    /// Lower the provided expression into the provided basic block, ensuring that the result of
    /// the expression is stored into the target. The resulting basic block of the control flow
    /// must be returned, even if it is unchanged.
    pub fn lower_expression(
        &self,
        bb: BasicBlockBuilder,
        expr: ast::Expression,
        target: Local,
    ) -> BasicBlockBuilder {
        match expr {
            ast::Expression::Infix(_) => todo!(),
            ast::Expression::Integer(i) => bb.statement(Statement::Assign(
                target,
                RValue::Scalar(Scalar::int(i.value)),
            )),
            ast::Expression::Boolean(_) => todo!(),
            ast::Expression::Ident(_) => todo!(),
            ast::Expression::Block(_) => todo!(),
            ast::Expression::If(_) => todo!(),
        }
    }
}

struct Function {
    /// Entry point for this function
    entry: BasicBlock,
}

#[cfg(test)]
mod test {
    use crate::core::ty::Ty;

    use super::*;

    #[test]
    fn lower_function() {
        /*
         * fn main() -> int {
         *     return 1;
         * }
         */
        let f = ast::Function {
            span: Default::default(),
            name: Default::default(),
            parameters: Vec::new(),
            return_ty: Ty::Int,
            body: ast::Block::new(&[ast::Statement::_return(ast::Expression::integer(1))]),
        };

        let (ir, ctx) = {
            let ctx = Context::new();

            (
                ctx.lower_function(f),
                Rc::into_inner(ctx.0).unwrap().into_inner(),
            )
        };

        /*
         * entry: {
         *     Assign(0, RValue::Scalar(1))
         *     Return
         * }
         */
        let entry_bb = &ctx.basic_blocks[ir.entry];

        assert_eq!(entry_bb.statements.len(), 1);
        assert!(matches!(
            entry_bb.statements[0],
            Statement::Assign(_, RValue::Scalar(_))
        ));

        if let Statement::Assign(target, RValue::Scalar(ref value)) = entry_bb.statements[0] {
            assert_eq!(target, RETURN_LOCAL);
            assert_eq!(value, Scalar::int(1));
        }
    }
}
