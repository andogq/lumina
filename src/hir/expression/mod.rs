use super::*;

mod array;
mod assign;
mod block;
mod boolean;
mod call;
mod cast;
mod ident;
mod if_else;
mod index;
mod infix;
mod integer;
mod loop_block;

pub use array::*;
pub use assign::*;
pub use block::*;
pub use boolean::*;
pub use call::*;
pub use cast::*;
pub use ident::*;
pub use if_else::*;
pub use index::*;
pub use infix::*;
pub use integer::*;
pub use loop_block::*;

ast_node! {
    Expression<M>(
        Array,
        Infix,
        Integer,
        Boolean,
        Ident,
        Block,
        If,
        Index,
        Call,
        Loop,
        Assign,
        Cast,
    )
}

impl<M: AstMetadata<Span = Span, TyInfo: Default>> Expression<M> {
    pub fn infix(left: Expression<M>, operation: InfixOperation, right: Expression<M>) -> Self {
        let span = left.span().start..right.span().end;
        Self::Infix(Infix::<M>::new(
            Box::new(left),
            operation,
            Box::new(right),
            span,
            M::TyInfo::default(),
        ))
    }

    pub fn integer(value: i64, span: Span) -> Self {
        Self::Integer(Integer::new(value, span, M::TyInfo::default()))
    }

    pub fn boolean(value: bool, span: Span) -> Self {
        Self::Boolean(Boolean::new(value, span, M::TyInfo::default()))
    }

    pub fn ident(name: M::IdentIdentifier, span: Span) -> Self {
        Self::Ident(Ident::new(name, span, M::TyInfo::default()))
    }

    pub fn block(statements: Vec<Statement<M>>, terminated: bool, span: Span) -> Self {
        Self::Block(Block::new(
            statements,
            terminated,
            span,
            M::TyInfo::default(),
        ))
    }

    pub fn _if(
        condition: Expression<M>,
        success: Block<M>,
        otherwise: Option<Block<M>>,
        span: Span,
    ) -> Self {
        Self::If(If::new(
            Box::new(condition),
            success,
            otherwise,
            span,
            M::TyInfo::default(),
        ))
    }

    pub fn call(identifier: M::FnIdentifier, args: Vec<Expression<M>>, span: Span) -> Self {
        Self::Call(Call::new(identifier, args, span, M::TyInfo::default()))
    }
}

impl<M: AstMetadata> Parsable for Expression<M> {
    fn register(parser: &mut Parser) {
        // Register all variant parsers
        Array::<UntypedAstMetadata>::register(parser);
        Assign::<UntypedAstMetadata>::register(parser);
        Block::<UntypedAstMetadata>::register(parser);
        Boolean::<UntypedAstMetadata>::register(parser);
        Call::<UntypedAstMetadata>::register(parser);
        Cast::<UntypedAstMetadata>::register(parser);
        Ident::<UntypedAstMetadata>::register(parser);
        If::<UntypedAstMetadata>::register(parser);
        Index::<UntypedAstMetadata>::register(parser);
        Infix::<UntypedAstMetadata>::register(parser);
        Integer::<UntypedAstMetadata>::register(parser);
        Loop::<UntypedAstMetadata>::register(parser);
    }
}

impl SolveType for Expression<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        Ok(match self {
            Expression::Infix(e) => Expression::Infix(e.solve(compiler, state)?),
            Expression::Integer(e) => Expression::Integer(e.solve(compiler, state)?),
            Expression::Boolean(e) => Expression::Boolean(e.solve(compiler, state)?),
            Expression::Ident(e) => Expression::Ident(e.solve(compiler, state)?),
            Expression::Block(e) => Expression::Block(e.solve(compiler, state)?),
            Expression::If(e) => Expression::If(e.solve(compiler, state)?),
            Expression::Index(e) => Expression::Index(e.solve(compiler, state)?),
            Expression::Loop(e) => Expression::Loop(e.solve(compiler, state)?),
            Expression::Call(e) => Expression::Call(e.solve(compiler, state)?),
            Expression::Assign(e) => Expression::Assign(e.solve(compiler, state)?),
            Expression::Cast(e) => Expression::Cast(e.solve(compiler, state)?),
            Expression::Array(e) => Expression::Array(e.solve(compiler, state)?),
        })
    }
}
