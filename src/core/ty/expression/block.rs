use std::collections::HashMap;

use crate::core::{
    ast::Block,
    symbol::Symbol,
    ty::{InferTy, Ty, TyError},
};

impl InferTy for Block {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        let mut ty = Ty::Unit;
        let mut ctx = symbols.clone();

        for statement in &self.statements {
            ty = statement.infer(&mut ctx)?;
        }

        Ok(ty)
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        let mut ty = None;
        let mut ctx = symbols.clone();

        for statement in &self.statements {
            let statement_ty = statement.return_ty(&mut ctx)?;

            match (ty, statement_ty) {
                (None, _) => {
                    ty = statement_ty;
                }
                (Some(ty), Some(statement_ty)) => {
                    if ty != statement_ty {
                        return Err(TyError::Mismatch(ty, statement_ty));
                    }
                }
                _ => {}
            }
        }

        Ok(ty)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        core::ast::{
            Boolean, Expression, ExpressionStatement, Integer, LetStatement, ReturnStatement,
            Statement,
        },
        util::source::Span,
    };

    use super::*;

    #[test]
    fn infer_block() {
        let b = Block {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Symbol::default(),
                    value: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                }),
                Statement::Expression(ExpressionStatement {
                    expression: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                    implicit_return: false,
                }),
                Statement::Return(ReturnStatement {
                    value: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                }),
            ],
        };

        assert_eq!(b.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
    #[test]
    fn return_block() {
        let b = Block {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Symbol::default(),
                    value: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                }),
                Statement::Expression(ExpressionStatement {
                    expression: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                    implicit_return: false,
                }),
                Statement::Return(ReturnStatement {
                    value: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                }),
            ],
        };

        assert_eq!(b.return_ty(&mut HashMap::new()).unwrap(), Some(Ty::Int));
    }

    #[test]
    fn return_block_conflicting_return() {
        let b = Block {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Symbol::default(),
                    value: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                }),
                Statement::Expression(ExpressionStatement {
                    expression: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                    implicit_return: false,
                }),
                Statement::Return(ReturnStatement {
                    value: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                }),
                Statement::Return(ReturnStatement {
                    value: Expression::Boolean(Boolean {
                        span: Span::default(),
                        value: true,
                    }),
                }),
            ],
        };

        assert!(b.return_ty(&mut HashMap::new()).is_err());
    }

    #[test]
    fn return_block_no_return() {
        let b = Block {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Symbol::default(),
                    value: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                }),
                Statement::Expression(ExpressionStatement {
                    expression: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                    implicit_return: false,
                }),
            ],
        };

        assert_eq!(b.return_ty(&mut HashMap::new()).unwrap(), None);
    }

    #[test]
    fn infer_block_expression() {
        // {
        //     let a = 1;
        //     1;
        // }
        let b = Block {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Symbol::default(),
                    value: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                }),
                Statement::Expression(ExpressionStatement {
                    expression: Expression::Integer(Integer {
                        span: Span::default(),
                        value: 1,
                    }),
                    implicit_return: false,
                }),
            ],
        };

        assert_eq!(b.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
}
