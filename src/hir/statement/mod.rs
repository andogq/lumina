mod s_break;
mod s_continue;
mod s_expression;
mod s_let;
mod s_return;

use super::*;

pub use self::{
    s_break::Break, s_continue::Continue, s_expression::ExpressionStatement, s_let::Let,
    s_return::Return,
};

ast_node! {
    Statement<M>(
        Return,
        Let,
        ExpressionStatement,
        Break,
        Continue,
    )
}

impl SolveType for Statement<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        Ok(match self {
            Statement::Return(s) => Statement::Return(s.solve(compiler, state)?),
            Statement::Let(s) => Statement::Let(s.solve(compiler, state)?),
            Statement::ExpressionStatement(s) => {
                Statement::ExpressionStatement(s.solve(compiler, state)?)
            }
            Statement::Break(s) => Statement::Break(s.solve(compiler, state)?),
            Statement::Continue(s) => Statement::Continue(s.solve(compiler, state)?),
        })
    }
}

impl<M: AstMetadata<TyInfo: Default>> Statement<M> {
    pub fn _return(expression: Expression<M>, span: M::Span) -> Self {
        Self::Return(Return::new(expression, span, M::TyInfo::default()))
    }

    pub fn _let(name: M::IdentIdentifier, value: Expression<M>, span: M::Span) -> Self {
        Self::Let(Let::new(name, value, span, M::TyInfo::default()))
    }

    pub fn expression(expression: Expression<M>, span: M::Span) -> Self {
        Self::ExpressionStatement(ExpressionStatement::new(
            expression,
            span,
            M::TyInfo::default(),
        ))
    }
}
