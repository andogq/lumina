use super::*;

impl parse_ast::Block {
    pub fn ty_solve(self, ctx: &mut FnCtx) -> Result<Block, TyError> {
        let statements = self
            .statements
            .into_iter()
            .map(|statement| statement.ty_solve(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let ty_info = TyInfo::try_from((
            // Type of this block will be the implicit return of the last block
            statements
                .last()
                // The block can only inherit the type of an expression statement
                .filter(|s| {
                    matches!(
                        s,
                        Statement::Expression(ExpressionStatement {
                            implicit_return: true,
                            ..
                        })
                    )
                })
                .map(|s| s.get_ty_info().ty)
                .unwrap_or(Ty::Unit),
            statements
                .iter()
                .map(|statement| statement.get_ty_info().return_ty),
        ))?;

        Ok(Block {
            span: self.span,
            statements,
            ty_info,
        })
    }
}

#[cfg(test)]
mod test {
    use string_interner::Symbol;

    use crate::{
        repr::{ast::untyped::*, ty::Ty},
        util::source::Span,
    };

    use super::expression::{FnCtx, TyError, TyInfo};

    fn run(b: Block) -> Result<TyInfo, TyError> {
        Ok(b.ty_solve(&mut FnCtx::mock())?.ty_info)
    }

    #[test]
    fn ty_check_block() {
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

        let ty_info = run(b).unwrap();

        assert_eq!(ty_info.ty, Ty::Unit);
        assert_eq!(ty_info.return_ty, Some(Ty::Int));
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

        assert!(run(b).is_err());
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

        let ty_info = run(b).unwrap();

        assert_eq!(ty_info.ty, Ty::Unit);
        assert_eq!(ty_info.return_ty, None);
    }
}
