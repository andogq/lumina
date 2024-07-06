use crate::core::ast::{parse_ast, ty_ast::*};

use super::{Ty, TyCtx, TyError};

impl parse_ast::Statement {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<Statement, TyError> {
        Ok(match self {
            parse_ast::Statement::Return(s) => Statement::Return(s.ty_solve(ctx)?),
            parse_ast::Statement::Let(s) => Statement::Let(s.ty_solve(ctx)?),
            parse_ast::Statement::Expression(s) => Statement::Expression(s.ty_solve(ctx)?),
        })
    }
}

impl parse_ast::LetStatement {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<LetStatement, TyError> {
        // Work out what the type of the value is
        let value = self.value.ty_solve(ctx)?;

        // Make sure the value type matches what the statement was annotated with
        if let Some(ty) = self.ty_info {
            let value_ty = value.get_ty_info();
            if ty != value_ty.ty {
                return Err(TyError::Mismatch(ty, value_ty.ty.clone()));
            }
        }

        // Record the type
        ctx.symbols
            .insert(self.name, value.get_ty_info().ty.clone());

        Ok(LetStatement {
            ty_info: TyInfo {
                ty: Ty::Unit,
                return_ty: value.get_ty_info().return_ty,
            },
            name: self.name,
            value,
            span: self.span,
        })
    }
}

impl parse_ast::ReturnStatement {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<ReturnStatement, TyError> {
        let value = self.value.ty_solve(ctx)?;

        Ok(ReturnStatement {
            ty_info: TyInfo::try_from((
                Ty::Unit,
                [Some(value.get_ty_info().ty), value.get_ty_info().return_ty],
            ))?,
            value,
            span: self.span,
        })
    }
}

impl parse_ast::ExpressionStatement {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<ExpressionStatement, TyError> {
        let expression = self.expression.ty_solve(ctx)?;

        // Expression statement has same type as the underlying expression
        let mut ty_info = expression.get_ty_info().clone();
        if !self.implicit_return {
            ty_info.ty = Ty::Unit;
        }

        Ok(ExpressionStatement {
            ty_info,
            expression,
            implicit_return: self.implicit_return,
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test_statement {
    use string_interner::Symbol;

    use crate::{
        core::{
            ast::parse_ast::*,
            ty::{Ty, TyCtx},
        },
        util::source::Span,
    };

    #[test]
    fn infer_return() {
        // return 0;
        let s = Statement::_return(Expression::integer(0, Span::default()), Span::default());

        assert_eq!(
            s.ty_solve(&mut Default::default())
                .unwrap()
                .get_ty_info()
                .ty,
            Ty::Unit
        );
    }
    #[test]
    fn return_return() {
        // return 0;
        let s = Statement::_return(Expression::integer(0, Span::default()), Span::default());

        assert_eq!(
            s.ty_solve(&mut Default::default())
                .unwrap()
                .get_ty_info()
                .return_ty,
            Some(Ty::Int)
        );
    }

    #[test]
    fn infer_let() {
        // let a = 0;
        let s = Statement::_let(
            Symbol::try_from_usize(0).unwrap(),
            Expression::integer(0, Span::default()),
            Span::default(),
        );

        let mut ctx = TyCtx::default();
        assert_eq!(s.ty_solve(&mut ctx).unwrap().get_ty_info().ty, Ty::Unit);
        assert_eq!(
            ctx.symbols
                .get(&Symbol::try_from_usize(0).unwrap())
                .cloned(),
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

        assert_eq!(
            s.ty_solve(&mut Default::default())
                .unwrap()
                .get_ty_info()
                .return_ty,
            None
        );
    }

    #[test]
    fn infer_expression() {
        // 0;
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            false,
            Span::default(),
        );

        assert_eq!(
            s.ty_solve(&mut Default::default())
                .unwrap()
                .get_ty_info()
                .ty,
            Ty::Unit
        );
    }
    #[test]
    fn return_expression() {
        // 0;
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            false,
            Span::default(),
        );

        assert_eq!(
            s.ty_solve(&mut Default::default())
                .unwrap()
                .get_ty_info()
                .return_ty,
            None
        );
    }

    #[test]
    fn infer_expression_implicit() {
        // 0
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            true,
            Span::default(),
        );

        assert_eq!(
            s.ty_solve(&mut Default::default())
                .unwrap()
                .get_ty_info()
                .ty,
            Ty::Int
        );
    }
    #[test]
    fn return_expression_implicit() {
        // 0
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            true,
            Span::default(),
        );

        assert_eq!(
            s.ty_solve(&mut Default::default())
                .unwrap()
                .get_ty_info()
                .return_ty,
            None
        );
    }
}
