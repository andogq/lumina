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
