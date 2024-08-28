use crate::{
    ast_node,
    repr::{ast::untyped::UntypedAstMetadata, ty::Ty},
    stage::type_check::TyError,
    util::scope::Scope,
};

use super::{expression::Block, SolveType};

ast_node! {
    Function<M> {
        name: M::FnIdentifier,
        parameters: Vec<(M::IdentIdentifier, Ty)>,
        return_ty: Ty,
        body: Block<M>,
        span,
    }
}

impl SolveType for Function<UntypedAstMetadata> {
    type State = ();

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        _state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        let idx = compiler
            .functions
            .get_idx(self.name)
            .expect("function must already be registered");

        // Create the scope for this function
        let mut scope = Scope::new();

        // Add all of the function's parameters into the scope so they're accessible
        let parameters = self
            .parameters
            .iter()
            .map(|(symbol, ty)| (scope.register(*symbol, ty.clone()), ty.clone()))
            .collect();

        // Type check the body, allowing it to use the function's scope
        let body = self.body.solve(compiler, &mut scope)?;

        // Access this function's registration
        let function = compiler
            .functions
            .get_mut(idx)
            .expect("function to be registered");

        // Add the bindings from the scope to the registration
        scope
            .into_iter()
            .for_each(|(binding, symbol, ty)| function.register_binding(binding, symbol, ty));

        // If the body contains any return statements, they must match the annotated return statement
        if let Some(return_ty) = &body.ty_info.return_ty {
            if !self.return_ty.check(return_ty) {
                return Err(TyError::Mismatch(self.return_ty, return_ty.clone()));
            }
        }

        // Ensure inferred return types match
        if !body.ty_info.ty.check(&Ty::Unit) && !self.return_ty.check(&body.ty_info.ty) {
            return Err(TyError::Mismatch(self.return_ty, body.ty_info.ty));
        }

        Ok(Function {
            name: idx,
            parameters,
            return_ty: self.return_ty,
            body,
            span: self.span,
        })
    }
}
