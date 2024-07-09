mod expression;
mod function;
mod program;
mod statement;

use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::core::ctx::Ctx;
use crate::repr::ty::Ty;
use crate::{core::ctx::Symbol, repr::ast::untyped as parse_ast};

#[derive(Clone, Debug)]
pub struct FunctionSignature {
    arguments: Vec<Ty>,
    return_ty: Ty,
}

impl<T> From<&crate::repr::ast::base::Function<T>> for FunctionSignature {
    fn from(function: &crate::repr::ast::base::Function<T>) -> Self {
        Self {
            arguments: function.parameters.iter().map(|(_, ty)| *ty).collect(),
            return_ty: function.return_ty,
        }
    }
}

pub struct TyCtx {
    ctx: Rc<RefCell<Ctx>>,
    function_signatures: HashMap<Symbol, FunctionSignature>,
}

impl TyCtx {
    pub fn new(ctx: Rc<RefCell<Ctx>>) -> Self {
        Self {
            ctx,
            function_signatures: HashMap::new(),
        }
    }

    pub fn mock() -> Self {
        let ctx = Rc::new(RefCell::new(Ctx::default()));
        Self::new(ctx)
    }
}

pub struct FnCtx {
    ty_ctx: Rc<RefCell<TyCtx>>,
    scope: HashMap<Symbol, Ty>,
}

impl FnCtx {
    pub fn new(ty_ctx: Rc<RefCell<TyCtx>>) -> Self {
        Self {
            ty_ctx,
            scope: HashMap::new(),
        }
    }

    pub fn mock() -> Self {
        Self::new(Rc::new(RefCell::new(TyCtx::mock())))
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
    fn collapse(mut iter: impl Iterator<Item = Ty>) -> Result<Option<Ty>, TyError> {
        iter.all_equal_value().map(Some).or_else(|e| match e {
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

use crate::repr::ast::typed::*;
