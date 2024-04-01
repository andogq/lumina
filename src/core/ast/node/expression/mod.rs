mod ident;
mod infix;
mod integer;

pub use ident::*;
pub use infix::*;
pub use integer::*;

pub enum Expression {
    Infix(Infix),
    Integer(Integer),
    Ident(Ident),
}
