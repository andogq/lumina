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
    use crate::core::ast::Expression;

    use super::*;

    #[test]
    fn infer_return() {
        // return 0;
        let s = Statement::_return(Expression::integer(0));

        assert_eq!(s.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
    #[test]
    fn return_return() {
        // return 0;
        let s = Statement::_return(Expression::integer(0));

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), Some(Ty::Int));
    }

    #[test]
    fn infer_let() {
        // let a = 0;
        let s = Statement::_let(Symbol::default(), Expression::integer(0));

        let mut symbols = HashMap::new();
        assert_eq!(s.infer(&mut symbols).unwrap(), Ty::Unit);
        assert_eq!(symbols.get(&Symbol::default()).cloned(), Some(Ty::Int));
    }
    #[test]
    fn return_let() {
        // let a = 0;
        let s = Statement::_let(Symbol::default(), Expression::integer(0));

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), None);
    }

    #[test]
    fn infer_expression() {
        // 0;
        let s = Statement::expression(Expression::integer(0), false);

        assert_eq!(s.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
    #[test]
    fn return_expression() {
        // 0;
        let s = Statement::expression(Expression::integer(0), false);

        assert_eq!(s.return_ty(&mut HashMap::new()).unwrap(), None);
    }

    #[test]
    fn infer_expression_implicit() {
        // 0
        let s = Statement::expression(Expression::integer(0), true);

        assert_eq!(s.infer(&mut HashMap::new()).unwrap(), Ty::Int);
    }
    #[test]
    fn return_expression_implicit() {
        // 0
        let s = Statement::expression(Expression::integer(0), true);

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
