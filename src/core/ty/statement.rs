use std::collections::HashMap;

use crate::core::ast::{ExpressionStatement, LetStatement, ReturnStatement, Statement};

use super::{InferTy, Symbol, Ty, TyError};

impl InferTy for Statement {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        match self {
            Statement::Expression(s) => s.infer(symbols),
            Statement::Let(s) => s.infer(symbols),
            Statement::Return(s) => s.infer(symbols),
        }
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        match self {
            Statement::Expression(s) => s.return_ty(symbols),
            Statement::Let(s) => s.return_ty(symbols),
            Statement::Return(s) => s.return_ty(symbols),
        }
    }
}

#[cfg(test)]
mod test_statement {
    use crate::{
        core::ast::{Expression, Integer},
        util::source::Span,
    };

    use super::*;

    #[test]
    fn infer_return() {
        let s = Statement::Return(ReturnStatement {
            value: Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            }),
        });

        assert_eq!(s.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
    #[test]
    fn return_return() {
        let s = Statement::Return(ReturnStatement {
            value: Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            }),
        });

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), Some(Ty::Int));
    }

    #[test]
    fn infer_let() {
        let s = Statement::Let(LetStatement {
            name: Symbol::default(),
            value: Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            }),
        });

        let mut symbols = HashMap::new();
        assert_eq!(s.infer(&mut symbols).unwrap(), Ty::Unit);
        assert_eq!(symbols.get(&Symbol::default()).cloned(), Some(Ty::Int));
    }
    #[test]
    fn return_let() {
        let s = Statement::Let(LetStatement {
            name: Symbol::default(),
            value: Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            }),
        });

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), None);
    }

    #[test]
    fn infer_expression() {
        let s = Statement::Expression(ExpressionStatement {
            expression: Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            }),
        });

        assert_eq!(s.infer(&mut HashMap::new()).unwrap(), Ty::Int);
    }
    #[test]
    fn return_expression() {
        let s = Statement::Expression(ExpressionStatement {
            expression: Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            }),
        });

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), None);
    }
}

impl InferTy for LetStatement {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        let ty = self.value.infer(symbols)?;
        symbols.insert(self.name, ty);
        Ok(Ty::Unit)
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        self.infer(symbols)?;

        self.value.return_ty(symbols)
    }
}

impl InferTy for ReturnStatement {
    fn infer(&self, _symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        // The return statement itself doesn't resolve to a type
        Ok(Ty::Unit)
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        Ok(Some(self.value.infer(symbols)?))
    }
}

impl InferTy for ExpressionStatement {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        self.expression.infer(symbols)
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        self.expression.return_ty(symbols)
    }
}
