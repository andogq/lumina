use ctx::TypeCheckCtx;

use crate::util::scope::Scope;

use super::*;

impl parse_ast::Statement {
    pub fn ty_solve(
        self,
        ctx: &mut impl TypeCheckCtx,
        scope: &mut Scope,
    ) -> Result<Statement, TyError> {
        Ok(match self {
            parse_ast::Statement::Return(s) => Statement::Return(s.ty_solve(ctx, scope)?),
            parse_ast::Statement::Let(s) => Statement::Let(s.ty_solve(ctx, scope)?),
            parse_ast::Statement::Expression(s) => Statement::Expression(s.ty_solve(ctx, scope)?),
        })
    }
}

impl parse_ast::LetStatement {
    pub fn ty_solve(
        self,
        ctx: &mut impl TypeCheckCtx,
        scope: &mut Scope,
    ) -> Result<LetStatement, TyError> {
        // Work out what the type of the value is
        let value = self.value.ty_solve(ctx, scope)?;

        // Make sure the value type matches what the statement was annotated with
        if let Some(ty) = self.ty_info {
            let value_ty = value.get_ty_info();
            if !ty.check(&value_ty.ty) {
                return Err(TyError::Mismatch(ty, value_ty.ty));
            }
        }

        // Record the type
        let binding = scope.register(self.binding, value.get_ty_info().ty);

        Ok(LetStatement {
            ty_info: TyInfo {
                ty: Ty::Unit,
                return_ty: value.get_ty_info().return_ty,
            },
            binding,
            value,
            span: self.span,
        })
    }
}

impl parse_ast::ReturnStatement {
    pub fn ty_solve(
        self,
        ctx: &mut impl TypeCheckCtx,
        scope: &mut Scope,
    ) -> Result<ReturnStatement, TyError> {
        let value = self.value.ty_solve(ctx, scope)?;

        Ok(ReturnStatement {
            ty_info: TyInfo::try_from((
                Ty::Never,
                [Some(value.get_ty_info().ty), value.get_ty_info().return_ty],
            ))?,
            value,
            span: self.span,
        })
    }
}

impl parse_ast::ExpressionStatement {
    pub fn ty_solve(
        self,
        ctx: &mut impl TypeCheckCtx,
        scope: &mut Scope,
    ) -> Result<ExpressionStatement, TyError> {
        let expression = self.expression.ty_solve(ctx, scope)?;

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

    use string_interner::Symbol as _;

    use crate::{
        repr::ast::untyped::*,
        stage::type_check::ctx::MockTypeCheckCtx,
        util::{scope::Scope, span::Span, symbol_map::interner_symbol_map::Symbol},
    };

    use super::Ty;

    #[test]
    fn return_statement() {
        // return 0;
        let s = Statement::_return(Expression::integer(0, Span::default()), Span::default());

        let ty_info = s
            .ty_solve(&mut MockTypeCheckCtx::new(), &mut Scope::new())
            .unwrap()
            .get_ty_info()
            .clone();

        assert_eq!(ty_info.ty, Ty::Never);
        assert_eq!(ty_info.return_ty, Some(Ty::Int));
    }

    #[test]
    fn let_statement() {
        // let a = 0;
        let s = Statement::_let(
            Symbol::try_from_usize(0).unwrap(),
            Expression::integer(0, Span::default()),
            Span::default(),
        );

        let mut scope = Scope::new();

        let ty_info = s
            .ty_solve(&mut MockTypeCheckCtx::new(), &mut scope)
            .unwrap()
            .get_ty_info()
            .clone();

        assert_eq!(ty_info.ty, Ty::Unit);
        assert_eq!(ty_info.return_ty, None);
        assert_eq!(
            scope.resolve(Symbol::try_from_usize(0).unwrap()).unwrap().1,
            Ty::Int
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

        let ty_info = s
            .ty_solve(&mut MockTypeCheckCtx::new(), &mut Scope::new())
            .unwrap()
            .get_ty_info()
            .clone();

        assert_eq!(ty_info.ty, Ty::Unit);
        assert_eq!(ty_info.return_ty, None);
    }

    #[test]
    fn infer_expression_implicit() {
        // 0
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            true,
            Span::default(),
        );

        let ty_info = s
            .ty_solve(&mut MockTypeCheckCtx::new(), &mut Scope::new())
            .unwrap()
            .get_ty_info()
            .clone();

        assert_eq!(ty_info.ty, Ty::Int);
        assert_eq!(ty_info.return_ty, None);
    }
}
