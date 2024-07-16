use super::*;

impl parse_ast::Function {
    pub fn ty_solve(self, ctx: Rc<RefCell<TyCtx>>) -> Result<Function, TyError> {
        // TODO: Register this symbol to get a function identifier for it
        let identifier = self.name;

        // Set up a fn ctx just for this function
        let mut ctx = FnCtx::new(ctx);

        // TODO: Need to insert parameters into scope

        let body = self.body.ty_solve(&mut ctx)?;

        // If the body contains any return statements, they must match the annotated return statement
        if let Some(return_ty) = body.ty_info.return_ty {
            if self.return_ty != return_ty {
                return Err(TyError::Mismatch(self.return_ty, return_ty));
            }
        }

        // Ensure inferred return types match
        if body.ty_info.ty != Ty::Unit && self.return_ty != body.ty_info.ty {
            return Err(TyError::Mismatch(self.return_ty, body.ty_info.ty));
        }

        Ok(Function {
            name: identifier,
            parameters: self.parameters,
            return_ty: self.return_ty,
            body,
            span: self.span,
        })
    }
}
