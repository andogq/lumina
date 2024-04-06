use std::collections::HashMap;

use crate::core::ast::Expression;

use super::{InferTy, Symbol, Ty, TyError};

mod block;
mod boolean;
mod ident;
mod if_else;
mod infix;
mod integer;

impl InferTy for Expression {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        match self {
            Expression::Infix(s) => s.infer(symbols),
            Expression::Integer(s) => s.infer(symbols),
            Expression::Boolean(s) => s.infer(symbols),
            Expression::Ident(s) => s.infer(symbols),
            Expression::Block(s) => s.infer(symbols),
            Expression::If(s) => s.infer(symbols),
        }
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        match self {
            Expression::Infix(s) => s.return_ty(symbols),
            Expression::Integer(s) => s.return_ty(symbols),
            Expression::Boolean(s) => s.return_ty(symbols),
            Expression::Ident(s) => s.return_ty(symbols),
            Expression::Block(s) => s.return_ty(symbols),
            Expression::If(s) => s.return_ty(symbols),
        }
    }
}
