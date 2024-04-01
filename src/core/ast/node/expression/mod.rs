mod boolean;
mod ident;
mod infix;
mod integer;

pub use boolean::*;
pub use ident::*;
pub use infix::*;
pub use integer::*;

pub enum Expression {
    Infix(Infix),
    Integer(Integer),
    Boolean(Boolean),
    Ident(Ident),
}
