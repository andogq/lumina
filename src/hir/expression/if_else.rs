use super::*;

ast_node! {
    If<M> {
        condition: Box<Expression<M>>,
        success: Block<M>,
        otherwise: Option<Block<M>>,
        span,
        ty_info,
    }
}

impl SolveType for If<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Make sure the condition is correctly typed
        let condition = self.condition.solve(compiler, state)?;
        let condition_ty = condition.get_ty_info();
        if !condition_ty.ty.check(&Ty::Boolean) {
            return Err(TyError::Mismatch(Ty::Boolean, condition_ty.ty.clone()));
        }

        let success = self.success.solve(compiler, state)?;
        let otherwise = self
            .otherwise
            .map(|otherwise| otherwise.solve(compiler, state))
            .transpose()?;

        let ty_info = TyInfo::try_from((
            // Branches must have the same type
            [
                success.ty_info.ty.clone(),
                otherwise
                    .as_ref()
                    .map(|otherwise| otherwise.ty_info.ty.clone())
                    .unwrap_or(Ty::Unit),
            ],
            // Any potential place for a return statement must be accounted for
            [
                condition_ty.return_ty.clone(),
                success.ty_info.return_ty.clone(),
                otherwise
                    .as_ref()
                    .and_then(|otherwise| otherwise.ty_info.return_ty.clone()),
            ],
        ))?;

        Ok(If {
            ty_info,
            condition: Box::new(condition),
            success,
            otherwise,
            span: self.span,
        })
    }
}
