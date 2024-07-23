use crate::repr::{
    ast::typed as ast,
    identifier::{FunctionIdx, ScopedBinding},
    ir::{BasicBlockIdx, Function, Triple, TripleRef},
    ty::Ty,
};

#[cfg_attr(test, mockall::automock(type FunctionBuilder = MockFunctionBuilder;))]
pub trait IRCtx {
    type FunctionBuilder: FunctionBuilder;

    /// Register an IR function implementation against it's identifier.
    fn register_function(&mut self, idx: FunctionIdx, function: Function);

    /// Prepare and return a new builder for the provided AST function representation.
    fn new_builder(&self, function: &ast::Function) -> Self::FunctionBuilder {
        Self::FunctionBuilder::new(function)
    }

    // TODO: Probably get rid of this
    fn all_functions(&self) -> Vec<(FunctionIdx, Function)>;
}

/// A stateful representation of a function that is being constructed. A function consists of basic
///  blocks, and the builder is always 'located' at a basic block.
pub trait FunctionBuilder {
    /// Initialise a new builder with the provided function, positioned at the entry point.
    fn new(function: &ast::Function) -> Self;

    /// Register the provided symbol with the given type into the function scope.
    fn register_scoped(&mut self, ident: ScopedBinding, ty: Ty);

    /// Add a triple to the current basic block.
    fn add_triple(&mut self, triple: Triple) -> TripleRef;

    /// Get the current basic block.
    fn current_bb(&self) -> BasicBlockIdx;

    /// Go to a specific basic block.
    fn goto_bb(&mut self, bb: BasicBlockIdx);

    /// Create a new basic block, switch to it, and return its identifier.
    fn push_bb(&mut self) -> BasicBlockIdx;

    /// Consume the builder, and register the built function against the context.
    fn build<Ctx: IRCtx<FunctionBuilder = Self>>(self, ctx: &mut Ctx);
}

#[cfg(test)]
mockall::mock! {
    pub FunctionBuilder {}

    impl FunctionBuilder for FunctionBuilder {
        fn new(function: &ast::Function) -> Self;
        fn register_scoped(&mut self, ident: ScopedBinding, ty: Ty);
        fn add_triple(&mut self, triple: Triple) -> TripleRef;
        fn current_bb(&self) -> BasicBlockIdx;
        fn goto_bb(&mut self, bb: BasicBlockIdx) ;
        fn push_bb(&mut self) -> BasicBlockIdx;
        #[mockall::concretize]
        fn build<Ctx: IRCtx<FunctionBuilder = MockFunctionBuilder>>(self, ctx: &mut Ctx);
    }
}
