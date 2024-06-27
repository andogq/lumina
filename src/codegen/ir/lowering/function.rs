use std::collections::HashMap;

use crate::{
    codegen::ir::{value::*, *},
    core::{
        ast::{self},
        symbol::Symbol,
    },
};

use self::function::Function;

#[derive(Clone)]
pub struct Scope {
    /// Locals contained within this scope
    pub locals: IndexVec<LocalDecl>,
    pub symbols: HashMap<Symbol, Local>,
}

impl Scope {
    /// Create a new instance of the scope.
    pub fn new() -> Self {
        Self {
            locals: {
                let mut locals = IndexVec::default();

                // Create a local for the return value
                locals.push(LocalDecl);

                locals
            },
            symbols: HashMap::new(),
        }
    }

    /// Nest a scope within this one.
    pub fn nest(&self) -> Self {
        self.clone()
    }

    // Create a new local within this scope.
    pub fn new_local(&mut self) -> Local {
        self.locals.push(LocalDecl)
    }

    pub fn register_symbol(&mut self, symbol: Symbol) -> Local {
        assert!(
            !self.symbols.contains_key(&symbol),
            "symbol {symbol:?} already has been registered"
        );

        let local = self.new_local();
        self.symbols.insert(symbol, local);
        local
    }

    pub fn get_symbol(&mut self, symbol: Symbol) -> Local {
        *self.symbols.get(&symbol).expect("symbol to be registered")
    }

    fn lower_block(
        &mut self,
        ctx: &Context,
        mut target: BasicBlockBuilder,
        block: ast::Block,
    ) -> BasicBlock {
        for s in block.statements {
            match s {
                ast::Statement::Return(s) => {
                    // Evaluate the expression into the return local
                    return self
                        .lower_expression(ctx, target, s.value, RETURN_LOCAL)
                        // Trigger the return terminator
                        .t_return();
                }
                ast::Statement::Let(s) => {
                    // Create a place for the variable
                    let local = self.register_symbol(s.name);

                    // Generate the IR for the value
                    target = self.lower_expression(ctx, target, s.value, local);
                }
                ast::Statement::Expression(_) => todo!(),
            }
        }

        panic!("implicit return unit value");
    }

    /// Lower the provided expression into the provided basic block, ensuring that the result of the
    /// expression is stored into the target. The resulting basic block of the control flow must be
    /// returned, even if it is unchanged.
    fn lower_expression(
        &mut self,
        _ctx: &Context,
        target: BasicBlockBuilder,
        expression: ast::Expression,
        local: Local,
    ) -> BasicBlockBuilder {
        match expression {
            ast::Expression::Infix(_) => todo!(),
            ast::Expression::Integer(i) => target.statement(Statement::Assign(
                local,
                RValue::Scalar(Scalar::int(i.value)),
            )),
            ast::Expression::Boolean(_) => todo!(),
            ast::Expression::Ident(ident) => target.statement(Statement::Load {
                result: local,
                target: self.get_symbol(ident.name),
            }),
            ast::Expression::Block(_) => todo!(),
            ast::Expression::If(_) => todo!(),
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

/// Lower the provided function AST node.
pub fn lower_function(ctx: &Context, node: ast::Function) -> Function {
    let mut scope = Scope::new();

    let entry = scope.lower_block(ctx, ctx.basic_block(), node.body);

    Function {
        entry,
        name: node.name,
        scope,
    }
}

#[cfg(test)]
mod test {
    use crate::core::ty::Ty;

    use super::*;

    #[test]
    fn return_integer() {
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
                lower_function(&ctx, f),
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
