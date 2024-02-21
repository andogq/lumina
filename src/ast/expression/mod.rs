mod boolean_literal;
mod identifier;
mod infix;
mod integer_literal;
mod prefix;

pub use boolean_literal::*;
pub use identifier::*;
pub use infix::*;
pub use integer_literal::*;
pub use prefix::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}
