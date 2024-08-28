use super::*;

ast_node! {
    Boolean<M> {
        value: bool,
        span,
        ty_info,
    }
}

impl SolveType for Boolean<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        _compiler: &mut crate::compiler::Compiler,
        _state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        Ok(Boolean {
            value: self.value,
            span: self.span,
            ty_info: TyInfo {
                ty: Ty::Boolean,
                return_ty: None,
            },
        })
    }
}

#[cfg(test)]
mod test_boolean {
    use super::*;

    #[test]
    fn boolean_infer() {
        assert_eq!(
            Boolean::new(false, Span::default(), Default::default())
                .solve(&mut Compiler::default(), &mut Scope::new())
                .unwrap()
                .ty_info
                .ty,
            Ty::Boolean
        );
    }

    #[test]
    fn boolean_return() {
        assert_eq!(
            Boolean::new(false, Span::default(), Default::default())
                .solve(&mut Compiler::default(), &mut Scope::new())
                .unwrap()
                .ty_info
                .return_ty,
            None,
        );
    }
}
