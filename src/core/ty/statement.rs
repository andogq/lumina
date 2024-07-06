use std::collections::HashMap;

use crate::core::ast::parse_ast::*;

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
        let ty = self.expression.infer(symbols);

        if self.implicit_return {
            ty
        } else {
            Ok(Ty::Unit)
        }
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        self.expression.return_ty(symbols)
    }
}

#[cfg(test)]
mod test_statement {
    use string_interner::Symbol;

    use crate::{core::ast::Expression, util::source::Span};

    use super::*;

    #[test]
    fn infer_return() {
        // return 0;
        let s = Statement::_return(Expression::integer(0, Span::default()), Span::default());

        assert_eq!(s.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
    #[test]
    fn return_return() {
        // return 0;
        let s = Statement::_return(Expression::integer(0, Span::default()), Span::default());

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), Some(Ty::Int));
    }

    #[test]
    fn infer_let() {
        // let a = 0;
        let s = Statement::_let(
            Symbol::try_from_usize(0).unwrap(),
            Expression::integer(0, Span::default()),
            Span::default(),
        );

        let mut symbols = HashMap::new();
        assert_eq!(s.infer(&mut symbols).unwrap(), Ty::Unit);
        assert_eq!(
            symbols.get(&Symbol::try_from_usize(0).unwrap()).cloned(),
            Some(Ty::Int)
        );
    }
    #[test]
    fn return_let() {
        // let a = 0;
        let s = Statement::_let(
            Symbol::try_from_usize(0).unwrap(),
            Expression::integer(0, Span::default()),
            Span::default(),
        );

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), None);
    }

    #[test]
    fn infer_expression() {
        // 0;
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            false,
            Span::default(),
        );

        assert_eq!(s.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
    #[test]
    fn return_expression() {
        // 0;
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            false,
            Span::default(),
        );

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), None);
    }

    #[test]
    fn infer_expression_implicit() {
        // 0
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            true,
            Span::default(),
        );

        assert_eq!(s.infer(&mut HashMap::new()).unwrap(), Ty::Int);
    }
    #[test]
    fn return_expression_implicit() {
        // 0
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            true,
            Span::default(),
        );

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), None);
    }
}
