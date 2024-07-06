mod expression;
mod function;
mod program;
mod statement;

use itertools::Itertools;
use std::collections::HashMap;

use crate::core::{parse::ast as parse_ast, symbol::Symbol};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    Boolean,
    Unit,
}

#[derive(Default)]
pub struct TyCtx {
    symbols: HashMap<Symbol, Ty>,
}

#[derive(Debug, thiserror::Error)]
pub enum TyError {
    #[error("mismatched types: {0:?} and {1:?}")]
    Mismatch(Ty, Ty),

    #[error("invalid return type, expected {expected:?} but found {found:?}")]
    Return {
        expected: Option<Ty>,
        found: Option<Ty>,
    },

    #[error("symbol not found: {0:?}")]
    SymbolNotFound(Symbol),
}

#[derive(Clone, Debug)]
pub struct TyInfo {
    pub ty: Ty,
    pub return_ty: Option<Ty>,
}

impl TyInfo {
    fn collapse(mut iter: impl Iterator<Item = Ty>) -> Result<Option<Ty>, TyError> {
        iter.all_equal_value()
            .map(|ty| Some(ty))
            .or_else(|e| match e {
                Some((ty1, ty2)) => Err(TyError::Mismatch(ty1, ty2)),
                None => Ok(None),
            })
    }
}

impl<TyIter, RetTyIter> TryFrom<(TyIter, RetTyIter)> for TyInfo
where
    TyIter: IntoIterator<Item = Ty>,
    RetTyIter: IntoIterator<Item = Option<Ty>>,
{
    type Error = TyError;

    fn try_from((ty_iter, return_ty_iter): (TyIter, RetTyIter)) -> Result<Self, Self::Error> {
        Ok(Self {
            // All of the provided types must match
            ty: TyInfo::collapse(ty_iter.into_iter())?.unwrap_or(Ty::Unit),
            return_ty: TyInfo::collapse(return_ty_iter.into_iter().flatten())?,
        })
    }
}

impl<RetTyIter> TryFrom<(Ty, RetTyIter)> for TyInfo
where
    RetTyIter: IntoIterator<Item = Option<Ty>>,
{
    type Error = TyError;

    fn try_from((ty, return_ty_iter): (Ty, RetTyIter)) -> Result<Self, Self::Error> {
        Ok(Self {
            ty,
            return_ty: TyInfo::collapse(return_ty_iter.into_iter().flatten())?,
        })
    }
}

impl FromIterator<TyInfo> for Result<TyInfo, TyError> {
    fn from_iter<T: IntoIterator<Item = TyInfo>>(iter: T) -> Self {
        let (ty_iter, return_ty_iter): (Vec<_>, Vec<_>) = iter
            .into_iter()
            .map(|ty_info| (ty_info.ty, ty_info.return_ty))
            .unzip();

        TyInfo::try_from((ty_iter.into_iter(), return_ty_iter.into_iter()))
    }
}

pub mod ast {
    use crate::generate_ast;

    use super::TyInfo;

    generate_ast!(TyInfo);
}

use ast::*;
