mod expression;
mod function;
mod program;
mod statement;

pub use expression::*;
pub use function::*;
pub use program::*;
pub use statement::*;

#[macro_export]
macro_rules! ast_node {
    (struct $struct_name:ident<$ty_info:ident> {  $($name:ident: $ty:ty,)* }) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name<$ty_info> {
            pub span: crate::util::source::Span,
            pub ty_info: $ty_info,
            $(pub $name: $ty,)*
        }

        impl<$ty_info: Default> $struct_name<$ty_info> {
            pub fn new($($name: $ty,)* span: crate::util::source::Span) -> Self {
                Self {
                    span,
                    ty_info: Default::default(),
                    $($name,)*
                }
            }
        }

        impl<$ty_info> crate::util::source::Spanned for $struct_name<$ty_info> {
            fn span(&self) -> &crate::util::source::Span {
                &self.span
            }
        }
    };

    (enum $enum_name:ident<$ty_info:ident> { $($name:ident($ty:ty),)* }) => {
        #[derive(Debug, Clone)]
        pub enum $enum_name<$ty_info> {
            $($name($ty),)*
        }

        impl<$ty_info> $enum_name<$ty_info> {
            pub fn get_ty_info(&self) -> &$ty_info {
                match self {
                    $(Self::$name(value) => &value.ty_info),*
                }
            }
        }

        impl<$ty_info> crate::util::source::Spanned for $enum_name<$ty_info> {
            fn span(&self) -> &crate::util::source::Span {
                match self {
                    $(Self::$name(value) => crate::util::source::Spanned::span(value)),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_ast {
    ($ty_info:ty) => {
        use crate::core::ast;

        // Re-export non-typed utilities
        pub use super::InfixOperation;

        pub type Block = ast::Block<$ty_info>;
        pub type Boolean = ast::Boolean<$ty_info>;
        pub type Ident = ast::Ident<$ty_info>;
        pub type If = ast::If<$ty_info>;
        pub type Infix = ast::Infix<$ty_info>;
        pub type Integer = ast::Integer<$ty_info>;
        pub type Expression = ast::Expression<$ty_info>;
        pub type Function = ast::Function<$ty_info>;
        pub type Program = ast::Program<$ty_info>;
        pub type Statement = ast::Statement<$ty_info>;
        pub type ReturnStatement = ast::ReturnStatement<$ty_info>;
        pub type LetStatement = ast::LetStatement<$ty_info>;
        pub type ExpressionStatement = ast::ExpressionStatement<$ty_info>;
    };
}

pub mod parse_ast {
    use crate::core::ty::Ty;

    generate_ast!(Option<Ty>);
}

pub mod ty_ast {
    use crate::core::ty::{Ty, TyError};

    use itertools::Itertools;

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

    generate_ast!(TyInfo);
}
