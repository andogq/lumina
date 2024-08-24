#[macro_export]
macro_rules! ast_node2 {
    ($name:ident<$metadata:ident> { $($tokens:tt)* }) => {
        ast_node2! { @ $name<$metadata> { $($tokens)* } -> () }
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
        ast_node2! {
            @ $name<$metadata> { $($tokens)* } -> (
                $($result)*
                span: $metadata::Span,
            )
        }
    };

    (@ $name:ident<$metadata:ident> { ty_info, $($tokens:tt)* } -> ( $($result:tt)*) ) => {
        ast_node2! {
            @ $name<$metadata> { $($tokens)* } -> (
                $($result)*
                ty_info: $metadata::TyInfo,
            )
        }
    };

    (@ $name:ident<$metadata:ident> { $field:ident: $ty:ty, $($tokens:tt)* } -> ( $($result:tt)*) ) => {
        ast_node2! {
            @ $name<$metadata> { $($tokens)* } -> (
                $($result)*
                $field: $ty,
            )
        }
    };
}

#[macro_export]
macro_rules! ast_node {
    // Common components for all variants of an AST node
    (common struct $struct_name:ident$(<$($generic:ident),*>)? {  $($name:ident: $ty:ty,)* }) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name$(<$($generic),*>)? {
            pub span: $crate::util::span::Span,
            $(pub $name: $ty,)*
        }
    };

    // AST node that is typed
    (typed struct $struct_name:ident<$ty_info:ident $(, $($generic:ident),*)?> { $($name:ident: $ty:ty,)* }) => {
        ast_node!(common struct $struct_name<$ty_info $(, $($generic),*)?> {
            $($name: $ty,)*
            ty_info: $ty_info,
        });

        impl<$ty_info: Default $(, $($generic),*)?> $struct_name<$ty_info $(, $($generic),*)?> {
            pub fn new($($name: $ty,)* span: $crate::util::span::Span) -> Self {
                Self {
                    span,
                    ty_info: Default::default(),
                    $($name,)*
                }
            }
        }
    };

    // AST node that contains no type information
    (struct $struct_name:ident$(<$($generic:ident),*>)? {  $($name:ident: $ty:ty,)* }) => {
        ast_node!(common struct $struct_name$(<$($generic),*>)? {
            $($name: $ty,)*
        });

        impl$(<$($generic),*>)? $struct_name$(<$($generic),*>)? {
            pub fn new($($name: $ty,)* span: $crate::util::span::Span) -> Self {
                Self {
                    span,
                    $($name,)*
                }
            }
        }
    };

    // AST node that consists of other AST nodes
    (enum $enum_name:ident<$ty_info:ident $(, $($generic:ident),*)?> { $($name:ident($ty:ty),)* }) => {
        #[derive(Debug, Clone)]
        pub enum $enum_name<$ty_info $(, $($generic),*)?> {
            $($name($ty),)*
        }

        impl<$ty_info $(, $($generic),*)?> $enum_name<$ty_info $(, $($generic),*)?> {
            pub fn get_ty_info(&self) -> &$ty_info {
                match self {
                    $(Self::$name(value) => &value.ty_info),*
                }
            }

            pub fn span(&self) -> &$crate::util::span::Span {
                match self {
                    $(Self::$name(value) => &value.span),*
                }
            }
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
