use ctx::TypeCheckCtx;

use crate::util::scope::Scope;

use super::*;

mod block;
mod boolean;
mod call;
mod ident;
mod if_else;
mod infix;
mod integer;

impl parse_ast::Expression {
    pub fn ty_solve(
        self,
        ctx: &mut impl TypeCheckCtx,
        scope: &mut Scope,
    ) -> Result<Expression, TyError> {
        Ok(match self {
            parse_ast::Expression::Infix(e) => Expression::Infix(e.ty_solve(ctx, scope)?),
            parse_ast::Expression::Integer(e) => Expression::Integer(e.ty_solve()?),
            parse_ast::Expression::Boolean(e) => Expression::Boolean(e.ty_solve()?),
            parse_ast::Expression::Ident(e) => Expression::Ident(e.ty_solve(ctx, scope)?),
            parse_ast::Expression::Block(e) => Expression::Block(e.ty_solve(ctx, scope)?),
            parse_ast::Expression::If(e) => Expression::If(e.ty_solve(ctx, scope)?),
            parse_ast::Expression::Call(e) => Expression::Call(e.ty_solve(ctx, scope)?),
        })
    }
}
