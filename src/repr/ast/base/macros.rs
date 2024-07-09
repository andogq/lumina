#[macro_export]
macro_rules! ast_node {
    (struct $struct_name:ident<$ty_info:ident> {  $($name:ident: $ty:ty,)* }) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name<$ty_info> {
            pub span: $crate::util::source::Span,
            pub ty_info: $ty_info,
            $(pub $name: $ty,)*
        }

        impl<$ty_info: Default> $struct_name<$ty_info> {
            pub fn new($($name: $ty,)* span: $crate::util::source::Span) -> Self {
                Self {
                    span,
                    ty_info: Default::default(),
                    $($name,)*
                }
            }
        }

        impl<$ty_info> $crate::util::source::Spanned for $struct_name<$ty_info> {
            fn span(&self) -> &$crate::util::source::Span {
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

        impl<$ty_info> $crate::util::source::Spanned for $enum_name<$ty_info> {
            fn span(&self) -> &$crate::util::source::Span {
                match self {
                    $(Self::$name(value) => $crate::util::source::Spanned::span(value)),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_ast {
    ($ty_info:ty) => {
        use $crate::repr::ast::base as ast;

        // Re-export non-typed utilities
        pub use ast::InfixOperation;

        pub type Block = ast::Block<$ty_info>;
        pub type Boolean = ast::Boolean<$ty_info>;
        pub type Call = ast::Call<$ty_info>;
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
