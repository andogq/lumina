use self::span::Span;

pub mod span;

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

pub trait GetAs<T>
where
    T: Name,
{
    fn get(self) -> Option<T>;
}

pub trait Name {
    fn name() -> &'static str;
}

token_enum! {
    Illegal: IllegalToken,
    EOF: EOFToken,

    Ident: IdentToken,
    Int: IntToken,
    String: StringToken,

    Assign: AssignToken,
    Plus: PlusToken,
    Minus: MinusToken,
    Bang: BangToken,
    Asterisk: AsteriskToken,
    Slash: SlashToken,

    LeftAngle: LeftAngleToken,
    RightAngle: RightAngleToken,

    Eq: EqToken,
    NotEq: NotEqToken,

    Comma: CommaToken,
    Semicolon: SemicolonToken,

    LeftParen: LeftParenToken,
    RightParen: RightParenToken,
    LeftBrace: LeftBraceToken,
    RightBrace: RightBraceToken,

    Function: FunctionToken,
    Let: LetToken,
    True: TrueToken,
    False: FalseToken,
    If: IfToken,
    Else: ElseToken,
    Return: ReturnToken
}

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

token!(IllegalToken);
token!(EOFToken);

token!(IdentToken { literal: String });
token!(IntToken { literal: String });
token!(StringToken { literal: String });

token!(AssignToken);
token!(PlusToken);
token!(MinusToken);
token!(BangToken);
token!(AsteriskToken);
token!(SlashToken);

token!(LeftAngleToken);
token!(RightAngleToken);

token!(EqToken);
token!(NotEqToken);

token!(CommaToken);
token!(SemicolonToken);

token!(LeftParenToken);
token!(RightParenToken);
token!(LeftBraceToken);
token!(RightBraceToken);

token!(FunctionToken);
token!(LetToken);
token!(TrueToken);
token!(FalseToken);
token!(IfToken);
token!(ElseToken);
token!(ReturnToken);
