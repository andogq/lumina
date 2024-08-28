#[macro_export]
macro_rules! ast_node {
    ($name:ident<$metadata:ident> { $($tokens:tt)* }) => {
        ast_node! { @ $name<$metadata> { $($tokens)* } -> () }
    };

    ($name:ident<$metadata:ident>( $($variant:ident,)* )) => {
        #[derive(Clone, Debug)]
        #[allow(clippy::enum_variant_names)]
        pub enum $name<$metadata: $crate::repr::ast::AstMetadata> {
            $($variant($variant<$metadata>),)*
        }

        impl<$metadata: $crate::repr::ast::AstMetadata> $name<$metadata> {
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

        ast_node!{ @trait $name<$metadata> }
    };

    (@ $name:ident<$metadata:ident> { } -> ( $($field:ident: $ty:ty,)*) ) => {
        #[derive(Clone, Debug)]
        pub struct $name<$metadata: $crate::repr::ast::AstMetadata> {
            $(pub $field: $ty,)*
        }

        impl<$metadata: $crate::repr::ast::AstMetadata> $name<$metadata> {
            pub fn new($($field: $ty,)*) -> Self {
                Self { $($field,)* }
            }
        }

        ast_node!{ @trait $name<$metadata> }
    };

    (@trait $name:ident<$metadata:ident>) => {
        impl $crate::hir::TypedAstNode for $name<$crate::repr::ast::typed::TypedAstMetadata> {
            type Untyped = $name<$crate::repr::ast::untyped::UntypedAstMetadata>;
        }

        impl $crate::hir::UntypedAstNode for $name<$crate::repr::ast::untyped::UntypedAstMetadata> {
            type Typed = $name<$crate::repr::ast::typed::TypedAstMetadata>;
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
        use $crate::hir;

        // Re-export non-typed utilities
        pub use hir::InfixOperation;

        pub type Array = hir::Array<$metadata>;
        pub type Block = hir::Block<$metadata>;
        pub type Boolean = hir::Boolean<$metadata>;
        pub type Call = hir::Call<$metadata>;
        pub type Ident = hir::Ident<$metadata>;
        pub type If = hir::If<$metadata>;
        pub type Loop = hir::Loop<$metadata>;
        pub type Index = hir::Index<$metadata>;
        pub type Infix = hir::Infix<$metadata>;
        pub type Integer = hir::Integer<$metadata>;
        pub type Assign = hir::Assign<$metadata>;
        pub type Cast = hir::Cast<$metadata>;
        pub type Expression = hir::Expression<$metadata>;
        pub type Function = hir::Function<$metadata>;
        pub type Program = hir::Program<$metadata>;
        pub type Statement = hir::Statement<$metadata>;
        pub type ReturnStatement = hir::Return<$metadata>;
        pub type LetStatement = hir::Let<$metadata>;
        pub type ExpressionStatement = hir::ExpressionStatement<$metadata>;
        pub type BreakStatement = hir::Break<$metadata>;
        pub type ContinueStatement = hir::Continue<$metadata>;
    };
}
