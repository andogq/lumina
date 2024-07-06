use std::collections::HashMap;

use crate::core::{
    ast::parse_ast::*,
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
    use string_interner::Symbol;

    use crate::{
        core::ast::{Expression, Statement},
        util::source::Span,
    };

    use super::*;

    #[test]
    fn infer_block() {
        // {
        //     let a = 1;
        //     1;
        //     return 1;
        // }
        let b = Block::new(
            vec![
                Statement::_let(
                    Symbol::try_from_usize(0).unwrap(),
                    Expression::integer(1, Span::default()),
                    Span::default(),
                ),
                Statement::expression(
                    Expression::integer(1, Span::default()),
                    false,
                    Span::default(),
                ),
                Statement::_return(Expression::integer(1, Span::default()), Span::default()),
            ],
            Span::default(),
        );

        assert_eq!(b.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
    #[test]
    fn return_block() {
        // {
        //     let a = 1;
        //     1;
        //     return 1;
        // }
        let b = Block::new(
            vec![
                Statement::_let(
                    Symbol::try_from_usize(0).unwrap(),
                    Expression::integer(1, Span::default()),
                    Span::default(),
                ),
                Statement::expression(
                    Expression::integer(1, Span::default()),
                    false,
                    Span::default(),
                ),
                Statement::_return(Expression::integer(1, Span::default()), Span::default()),
            ],
            Span::default(),
        );

        assert_eq!(b.return_ty(&mut HashMap::new()).unwrap(), Some(Ty::Int));
    }

    #[test]
    fn return_block_conflicting_return() {
        // {
        //     let a = 1;
        //     1;
        //     return 1;
        //     return true;
        // }
        let b = Block::new(
            vec![
                Statement::_let(
                    Symbol::try_from_usize(0).unwrap(),
                    Expression::integer(1, Span::default()),
                    Span::default(),
                ),
                Statement::expression(
                    Expression::integer(1, Span::default()),
                    false,
                    Span::default(),
                ),
                Statement::_return(Expression::integer(1, Span::default()), Span::default()),
                Statement::_return(Expression::boolean(true, Span::default()), Span::default()),
            ],
            Span::default(),
        );

        assert!(b.return_ty(&mut HashMap::new()).is_err());
    }

    #[test]
    fn return_block_no_return() {
        // {
        //     let a = 1;
        //     1;
        // }
        let b = Block::new(
            vec![
                Statement::_let(
                    Symbol::try_from_usize(0).unwrap(),
                    Expression::integer(1, Span::default()),
                    Span::default(),
                ),
                Statement::expression(
                    Expression::integer(1, Span::default()),
                    false,
                    Span::default(),
                ),
            ],
            Span::default(),
        );

        assert_eq!(b.return_ty(&mut HashMap::new()).unwrap(), None);
    }

    #[test]
    fn infer_block_expression() {
        // {
        //     let a = 1;
        //     1;
        // }
        let b = Block::new(
            vec![
                Statement::_let(
                    Symbol::try_from_usize(0).unwrap(),
                    Expression::integer(1, Span::default()),
                    Span::default(),
                ),
                Statement::expression(
                    Expression::integer(1, Span::default()),
                    false,
                    Span::default(),
                ),
            ],
            Span::default(),
        );

        assert_eq!(b.infer(&mut HashMap::new()).unwrap(), Ty::Unit);
    }
}
