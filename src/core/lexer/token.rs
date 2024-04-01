use crate::util::source::Span;

/// Creates a struct for a token without having to repeat all of the boiler plate, namely the span
/// for each token.
macro_rules! token {
    ($name:ident { $($field:ident: $value:ty),* }) => {
        token!(struct $name { $($field: $value,)* });

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                token!(condition $(self.$field == other.$field);*)
            }
        }
    };

    ($name:ident) => {
        token!(struct $name { });

        impl PartialEq for $name {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }
    };

    (struct $name:ident { $($field:ident: $value:ty,)* }) => {
        #[derive(Clone, Debug, Default)]
        pub struct $name {
            pub span: Span,
            $(pub $field: $value,)*
        }
    };

    (condition $field:expr) => {
        $field
    };

    (condition $field:expr; $($tail:tt)*) => {
        $field && token!(condition $($tail)*)
    };
}

/// Utility macro to easily create a large enum to contain each token variant.
macro_rules! token_enum {
    ($($name:ident: $token:ty),*) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum Token {
            $($name($token)),*
        }

        $(
        impl GetAs<$token> for Token {
            fn get(self) -> Option<$token> {
                if let Self::$name(token) = self {
                    Some(token)
                } else {
                    None
                }
            }
        }

        impl Name for $token {
            fn name() -> &'static str {
                stringify!($name)
            }
        }
        )*
    };
}

/// Convinience trait to produce a specific token variant from the mega token struct.
pub trait GetAs<T>
where
    T: Name,
{
    fn get(self) -> Option<T>;
}

/// Trait to produce the token's name.
pub trait Name {
    fn name() -> &'static str;
}

token!(IllegalToken);
token!(EOFToken);

token!(IntegerToken { literal: String });
token!(IdentToken { literal: String });

token!(PlusToken);

token!(SemicolonToken);
token!(ThinArrowToken);

token!(LeftParenToken);
token!(RightParenToken);
token!(LeftBraceToken);
token!(RightBraceToken);

token!(FnToken);
token!(ReturnToken);

token_enum! {
    Illegal: IllegalToken,
    EOF: EOFToken,

    Integer: IntegerToken,
    Ident: IdentToken,

    Plus: PlusToken,

    Semicolon: SemicolonToken,
    ThinArrow: ThinArrowToken,

    LeftParen: LeftParenToken,
    RightParen: RightParenToken,
    LeftBrace: LeftBraceToken,
    RightBrace: RightBraceToken,

    Fn: FnToken,
    Return: ReturnToken
}
