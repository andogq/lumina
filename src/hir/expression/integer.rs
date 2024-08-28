use super::*;

ast_node! {
    Integer<M> {
        value: i64,
        span,
        ty_info,
    }
}

impl SolveType for Integer<UntypedAstMetadata> {
    type State = Scope;

    fn solve(self, _compiler: &mut Compiler, _scope: &mut Scope) -> Result<Self::Typed, TyError> {
        Ok(Integer {
            value: self.value,
            span: self.span,
            ty_info: TyInfo {
                ty: Ty::Int,
                return_ty: None,
            },
        })
    }
}

#[cfg(test)]
mod test_integer {
    use super::*;

    #[test]
    fn integer_infer() {
        assert_eq!(
            Integer::new(0, Span::default(), Default::default())
                .solve(&mut Compiler::default(), &mut Scope::new())
                .unwrap()
                .ty_info
                .ty,
            Ty::Int
        );
    }

    #[test]
    fn integer_return() {
        assert_eq!(
            Integer::new(0, Span::default(), Default::default())
                .solve(&mut Compiler::default(), &mut Scope::new())
                .unwrap()
                .ty_info
                .return_ty,
            None,
        );
    }
}
