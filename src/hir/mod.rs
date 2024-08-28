mod expression;
mod function;
mod program;
mod statement;

use crate::{
    ast_node,
    compiler::Compiler,
    repr::{
        ast::{typed::TyInfo, untyped::UntypedAstMetadata, AstMetadata},
        token::Token,
        ty::Ty,
    },
    stage::type_check::TyError,
    util::{scope::Scope, span::Span},
};

pub use expression::*;
pub use function::*;
pub use program::*;
pub use statement::*;

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
