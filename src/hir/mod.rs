mod expression;
mod function;
mod program;
mod statement;

use crate::{
    ast_node,
    compiler::Compiler,
    repr::{
        ast::{untyped::UntypedAstMetadata, AstMetadata},
        token::Token,
    },
    stage::parse::{parser::Parser, Lexer, ParseError, Precedence},
    ty::{Ty, TyError, TyInfo},
    util::{scope::Scope, span::Span},
};

pub use expression::*;
pub use function::*;
pub use program::*;
pub use statement::*;

#[allow(dead_code)]
pub trait Parsable {
    /// Register the parser for this node against the provided parser.
    fn register(parser: &mut Parser);
}

pub trait SolveType: UntypedAstNode {
    type State;

    fn solve(
        self,
        compiler: &mut Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError>;
}

pub trait UntypedAstNode {
    type Typed: TypedAstNode;
}

#[allow(dead_code)]
pub trait TypedAstNode {
    type Untyped: UntypedAstNode;
}
