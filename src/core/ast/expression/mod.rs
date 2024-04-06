mod block;
mod boolean;
mod ident;
mod if_else;
mod infix;
mod integer;

pub use block::*;
pub use boolean::*;
pub use ident::*;
pub use if_else::*;
pub use infix::*;
pub use integer::*;

#[derive(Debug)]
pub enum Expression {
    Infix(Infix),
    Integer(Integer),
    Boolean(Boolean),
    Ident(Ident),
    Block(Block),
    If(If),
}
