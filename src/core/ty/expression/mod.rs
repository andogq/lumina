use crate::core::ast::{parse_ast, ty_ast::*};

use super::{TyCtx, TyError};

mod block;
mod boolean;
mod ident;
mod if_else;
mod infix;
mod integer;

impl parse_ast::Expression {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<Expression, TyError> {
        Ok(match self {
            parse_ast::Expression::Infix(e) => Expression::Infix(e.ty_solve(ctx)?),
            parse_ast::Expression::Integer(e) => Expression::Integer(e.ty_solve()?),
            parse_ast::Expression::Boolean(e) => Expression::Boolean(e.ty_solve()?),
            parse_ast::Expression::Ident(e) => Expression::Ident(e.ty_solve(ctx)?),
            parse_ast::Expression::Block(e) => Expression::Block(e.ty_solve(ctx)?),
            parse_ast::Expression::If(e) => Expression::If(e.ty_solve(ctx)?),
        })
    }
}
