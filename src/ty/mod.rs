mod error;
mod function;
mod ty_info;

use std::ops::Range;

use crate::{hir::Parsable, repr::token::Token, stage::parse::parser::Parser};

pub use self::{error::TyError, function::FunctionSignature, ty_info::TyInfo};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    Uint,
    Boolean,
    Unit,
    Never,
    Array { inner: Box<Ty>, size: u32 },
}

impl Ty {
    pub fn check(&self, other: &Ty) -> bool {
        match (self, other) {
            (lhs, rhs) if lhs == rhs => true,
            (Ty::Never, _) | (_, Ty::Never) => true,
            _ => false,
        }
    }
}

pub struct TySpanned {
    pub ty: Ty,
    pub span: Range<usize>,
}

impl Parsable for TySpanned {
    fn register(parser: &mut Parser) {
        [
            (Token::Int, Ty::Int),
            (Token::Uint, Ty::Uint),
            (Token::Bool, Ty::Boolean),
        ]
        .into_iter()
        .for_each(|(token, ty)| {
            assert!(parser.register_prefix(token, move |_, _, lexer| {
                let (_, span) = lexer.next_spanned().unwrap();

                Ok(TySpanned {
                    ty: ty.clone(),
                    span,
                })
            }));
        });
    }
}
