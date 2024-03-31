mod infix;
mod integer;

pub use infix::*;
pub use integer::*;

pub enum Expression {
    Infix(Infix),
    Integer(Integer),
}
