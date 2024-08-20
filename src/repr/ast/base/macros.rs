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
    (TyInfo: $ty_info:ty, FnIdentifier: $fn_identifier:ty, IdentIdentifier: $ident_identifier:ty) => {
        use $crate::repr::ast::base as ast;

        // Re-export non-typed utilities
        pub use ast::InfixOperation;

        pub type Block = ast::Block<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Boolean = ast::Boolean<$ty_info>;
        pub type Call = ast::Call<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Ident = ast::Ident<$ty_info, $ident_identifier>;
        pub type If = ast::If<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Loop = ast::Loop<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Infix = ast::Infix<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Integer = ast::Integer<$ty_info>;
        pub type Assign = ast::Assign<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Expression = ast::Expression<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Function = ast::Function<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Program = ast::Program<$ty_info, $fn_identifier, $ident_identifier>;
        pub type Statement = ast::Statement<$ty_info, $fn_identifier, $ident_identifier>;
        pub type ReturnStatement =
            ast::ReturnStatement<$ty_info, $fn_identifier, $ident_identifier>;
        pub type LetStatement = ast::LetStatement<$ty_info, $fn_identifier, $ident_identifier>;
        pub type ExpressionStatement =
            ast::ExpressionStatement<$ty_info, $fn_identifier, $ident_identifier>;
        pub type BreakStatement = ast::BreakStatement<$ty_info>;
        pub type ContinueStatement = ast::ContinueStatement<$ty_info>;
    };
}
