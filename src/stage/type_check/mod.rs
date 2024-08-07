mod ctx;
mod expression;
mod function;
mod program;
mod statement;

use itertools::Itertools;

use crate::repr::ty::Ty;
use crate::{
    repr::ast::{base as base_ast, untyped as parse_ast},
    util::symbol_map::interner_symbol_map::Symbol,
};

pub use ctx::TypeCheckCtx;

#[derive(Clone, Debug)]
pub struct FunctionSignature {
    pub arguments: Vec<Ty>,
    pub return_ty: Ty,
}

impl<TyInfo, FnIdentifier, IdentIdentifier>
    From<&base_ast::Function<TyInfo, FnIdentifier, IdentIdentifier>> for FunctionSignature
{
    fn from(function: &base_ast::Function<TyInfo, FnIdentifier, IdentIdentifier>) -> Self {
        Self {
            arguments: function.parameters.iter().map(|(_, ty)| *ty).collect(),
            return_ty: function.return_ty,
        }
    }
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

impl TyInfo {
    fn collapse(iter: impl Iterator<Item = Ty>) -> Result<Option<Ty>, TyError> {
        // Filter out all instances of `never` type, as it could be any type
        iter.filter(|ty| !matches!(ty, Ty::Never))
            .all_equal_value()
            .map(Some)
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
            ty: TyInfo::collapse(ty_iter.into_iter())?.unwrap_or(Ty::Never),
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

use crate::repr::ast::typed::*;
