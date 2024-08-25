#[macro_export]
macro_rules! ast_node {
    ($name:ident<$metadata:ident> { $($tokens:tt)* }) => {
        ast_node! { @ $name<$metadata> { $($tokens)* } -> () }
    };

    ($name:ident<$metadata:ident>( $($variant:ident,)* )) => {
        #[derive(Clone, Debug)]
        pub enum $name<$metadata: $crate::repr::ast::base::AstMetadata> {
            $($variant($variant<$metadata>),)*
        }

        impl<$metadata: $crate::repr::ast::base::AstMetadata> $name<$metadata> {
            pub fn get_ty_info(&self) -> &$metadata::TyInfo {
                match self {
                    $(Self::$variant(value) => &value.ty_info),*
                }
            }

            pub fn span(&self) -> &$metadata::Span {
                match self {
                    $(Self::$variant(value) => &value.span),*
                }
            }
        }
    };

    (@ $name:ident<$metadata:ident> { } -> ( $($field:ident: $ty:ty,)*) ) => {
        #[derive(Clone, Debug)]
        pub struct $name<$metadata: $crate::repr::ast::base::AstMetadata> {
            $(pub $field: $ty,)*
        }

        impl<$metadata: $crate::repr::ast::base::AstMetadata> $name<$metadata> {
            pub fn new($($field: $ty,)*) -> Self {
                Self { $($field,)* }
            }
        }
    };

    (@ $name:ident<$metadata:ident> { span, $($tokens:tt)* } -> ( $($result:tt)*) ) => {
        ast_node! {
            @ $name<$metadata> { $($tokens)* } -> (
                $($result)*
                span: $metadata::Span,
            )
        }
    };

    (@ $name:ident<$metadata:ident> { ty_info, $($tokens:tt)* } -> ( $($result:tt)*) ) => {
        ast_node! {
            @ $name<$metadata> { $($tokens)* } -> (
                $($result)*
                ty_info: $metadata::TyInfo,
            )
        }
    };

    (@ $name:ident<$metadata:ident> { $field:ident: $ty:ty, $($tokens:tt)* } -> ( $($result:tt)*) ) => {
        ast_node! {
            @ $name<$metadata> { $($tokens)* } -> (
                $($result)*
                $field: $ty,
            )
        }
    };
}

#[macro_export]
macro_rules! generate_ast {
    ($metadata:ty) => {
        use $crate::repr::ast::base as ast;

        // Re-export non-typed utilities
        pub use ast::InfixOperation;

        pub type Block = ast::Block<$metadata>;
        pub type Boolean = ast::Boolean<$metadata>;
        pub type Call = ast::Call<$metadata>;
        pub type Ident = ast::Ident<$metadata>;
        pub type If = ast::If<$metadata>;
        pub type Loop = ast::Loop<$metadata>;
        pub type Infix = ast::Infix<$metadata>;
        pub type Integer = ast::Integer<$metadata>;
        pub type Assign = ast::Assign<$metadata>;
        pub type Cast = ast::Cast<$metadata>;
        pub type Expression = ast::Expression<$metadata>;
        pub type Function = ast::Function<$metadata>;
        pub type Program = ast::Program<$metadata>;
        pub type Statement = ast::Statement<$metadata>;
        pub type ReturnStatement = ast::Return<$metadata>;
        pub type LetStatement = ast::Let<$metadata>;
        pub type ExpressionStatement = ast::ExpressionStatement<$metadata>;
        pub type BreakStatement = ast::Break<$metadata>;
        pub type ContinueStatement = ast::Continue<$metadata>;
    };
}
