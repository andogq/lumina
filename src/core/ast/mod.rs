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

            pub fn dummy($($name: $ty,)*) -> Self {
                Self {
                    span: Default::default(),
                    ty_info: Default::default(),
                    $($name),*
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
