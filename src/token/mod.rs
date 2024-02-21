#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    /// Illegal/unknown token
    Illegal(IllegalToken),
    /// End of file
    EOF(EOFToken),

    // Identifiers and literals
    Ident(IdentToken),
    Int(IntToken),

    // Operators
    Assign(AssignToken),
    Plus(PlusToken),
    Minus(MinusToken),
    Bang(BangToken),
    Asterisk(AsteriskToken),
    Slash(SlashToken),

    LeftAngle(LeftAngleToken),
    RightAngle(RightAngleToken),

    Eq(EqToken),
    NotEq(NotEqToken),

    // Delimiters
    Comma(CommaToken),
    Semicolon(SemicolonToken),

    // Brackets
    LeftParen(LeftParenToken),
    RightParen(RightParenToken),
    LeftBrace(LeftBraceToken),
    RightBrace(RightBraceToken),

    // Keywords
    Function(FunctionToken),
    Let(LetToken),
    True(TrueToken),
    False(FalseToken),
    If(IfToken),
    Else(ElseToken),
    Return(ReturnToken),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IllegalToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EOFToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentToken {
    pub literal: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntToken {
    pub literal: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlusToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinusToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BangToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AsteriskToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeftAngleToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RightAngleToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EqToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotEqToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommaToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemicolonToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeftParenToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RightParenToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeftBraceToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RightBraceToken;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrueToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FalseToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElseToken;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnToken;
