use ctx::TypeCheckCtx;

use crate::util::scope::Scope;

use super::*;

impl parse_ast::Function {
    pub fn ty_solve(self, ctx: &mut impl TypeCheckCtx) -> Result<Function, TyError> {
        let identifier = ctx
            .lookup_function_symbol(self.name)
            .expect("function must already be registered");

        // Create the scope for this function
        let mut scope = Scope::new();

        // Add all of the function's parameters into the scope so they're accessible
        let parameters = self
            .parameters
            .iter()
            .map(|(symbol, ty)| (scope.register(*symbol, *ty), *ty))
            .collect();

        // Type check the body, allowing it to use the function's scope
        let body = self.body.ty_solve(ctx, &mut scope)?;

        // Store the function's scope for future reference
        ctx.set_function_scope(identifier, scope);

        // If the body contains any return statements, they must match the annotated return statement
        if let Some(return_ty) = body.ty_info.return_ty {
            if !self.return_ty.check(&return_ty) {
                return Err(TyError::Mismatch(self.return_ty, return_ty));
            }
        }

        // Ensure inferred return types match
        if !body.ty_info.ty.check(&Ty::Unit) && !self.return_ty.check(&body.ty_info.ty) {
            return Err(TyError::Mismatch(self.return_ty, body.ty_info.ty));
        }

        Ok(Function {
            name: identifier,
            parameters,
            return_ty: self.return_ty,
            body,
            span: self.span,
        })
    }
}
