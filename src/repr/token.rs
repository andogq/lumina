use std::fmt::Display;

use crate::util::source::{Span, Spanned};

/// Creates a struct for a token without having to repeat all of the boiler plate, namely the span
/// for each token.
macro_rules! token {
    ($name:ident { $($field:ident: $value:ty),* }, $display:expr) => {
        token!($name { $($field: $value),* });
        token!(display $name $display);
    };

    ($name:ident { $($field:ident: $value:ty),* }) => {
        token!(struct $name { $($field: $value,)* });

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                token!(condition $(self.$field == other.$field);*)
            }
        }
    };

    ($name:ident, $display:expr) => {
        token!($name);
        token!(display $name $display);
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

        impl Spanned for $name {
            fn span(&self) -> &Span {
                &self.span
            }
        }
    };

    (display $name:ident $display:expr) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                $display.fmt(f)
            }
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

        impl Spanned for Token {
            fn span(&self) -> &Span {
                match self {
                    $(Self::$name(token) => token.span()),*
                }
            }
        }

        impl std::fmt::Display for Token {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    $(Self::$name(token) => token.fmt(f)),*
                }
            }
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

token!(IllegalToken { c: char });
impl Display for IllegalToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.c.fmt(f)
    }
}

token!(EOFToken, "<EOF>");

token!(IntegerToken { literal: String }, "integer");
token!(IdentToken { literal: String }, "ident");

token!(MinusToken, "-");
token!(PlusToken, "+");
token!(EqToken, "==");
token!(NotEqToken, "!=");

token!(EqualsToken, "=");

token!(ColonToken, ":");
token!(SemicolonToken, ";");
token!(ThinArrowToken, "->");
token!(CommaToken, ",");

token!(LeftParenToken, "(");
token!(RightParenToken, ")");
token!(LeftBraceToken, "{");
token!(RightBraceToken, "}");

token!(TrueToken, "true");
token!(FalseToken, "false");

token!(FnToken, "fn");
token!(ReturnToken, "return");
token!(LetToken, "let");
token!(IfToken, "if");
token!(ElseToken, "else");

token_enum! {
    Illegal: IllegalToken,
    EOF: EOFToken,

    Integer: IntegerToken,
    Ident: IdentToken,

    Minus: MinusToken,
    Plus: PlusToken,
    Eq: EqToken,
    NotEq: NotEqToken,

    Equals: EqualsToken,

    Colon: ColonToken,
    Semicolon: SemicolonToken,
    ThinArrow: ThinArrowToken,
    Comma: CommaToken,

    LeftParen: LeftParenToken,
    RightParen: RightParenToken,
    LeftBrace: LeftBraceToken,
    RightBrace: RightBraceToken,

    True: TrueToken,
    False: FalseToken,

    Fn: FnToken,
    Return: ReturnToken,
    Let: LetToken,
    If: IfToken,
    Else: ElseToken
}

impl Token {
    pub fn integer(literal: &str) -> Self {
        Self::Integer(IntegerToken {
            literal: literal.to_string(),
            ..Default::default()
        })
    }
    pub fn ident(literal: &str) -> Self {
        Self::Ident(IdentToken {
            literal: literal.to_string(),
            ..Default::default()
        })
    }

    pub fn plus() -> Self {
        Self::Plus(Default::default())
    }
    pub fn eq() -> Self {
        Self::Eq(Default::default())
    }
    pub fn not_eq() -> Self {
        Self::NotEq(Default::default())
    }

    pub fn equals() -> Self {
        Self::Equals(Default::default())
    }

    pub fn colon() -> Self {
        Self::Colon(Default::default())
    }
    pub fn semicolon() -> Self {
        Self::Semicolon(Default::default())
    }
    pub fn thin_arrow() -> Self {
        Self::ThinArrow(Default::default())
    }
    pub fn comma() -> Self {
        Self::Comma(Default::default())
    }

    pub fn left_paren() -> Self {
        Self::LeftParen(Default::default())
    }
    pub fn right_paren() -> Self {
        Self::RightParen(Default::default())
    }
    pub fn left_brace() -> Self {
        Self::LeftBrace(Default::default())
    }
    pub fn right_brace() -> Self {
        Self::RightBrace(Default::default())
    }

    pub fn t_true() -> Self {
        Self::True(Default::default())
    }
    pub fn t_false() -> Self {
        Self::False(Default::default())
    }

    pub fn t_fn() -> Self {
        Self::Fn(Default::default())
    }
    pub fn t_return() -> Self {
        Self::Return(Default::default())
    }
    pub fn t_let() -> Self {
        Self::Let(Default::default())
    }
    pub fn t_if() -> Self {
        Self::If(Default::default())
    }
    pub fn t_else() -> Self {
        Self::Else(Default::default())
    }
}

/// Additional 'helper' token that may be either a true or false token.
pub enum BooleanToken {
    True(TrueToken),
    False(FalseToken),
}
