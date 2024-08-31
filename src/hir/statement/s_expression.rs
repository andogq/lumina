use super::*;

ast_node! {
    ExpressionStatement<M> {
        expression: Expression<M>,
        implicit_return: bool,
        span,
        ty_info,
    }
}

// TODO: Need a 'fallback' parser option when registering

impl SolveType for ExpressionStatement<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        let expression = self.expression.solve(compiler, state)?;

        // Expression statement has same type as the underlying expression
        let mut ty_info = expression.get_ty_info().clone();
        if !self.implicit_return {
            ty_info.ty = Ty::Unit;
        }

        Ok(ExpressionStatement {
            ty_info,
            expression,
            implicit_return: self.implicit_return,
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    mod parse {
        use super::*;
    }

    mod ty {
        use super::*;

        #[rstest]
        #[case(false, Ty::Unit)]
        #[case(true, Ty::Int)]
        fn infer(#[case] implicit: bool, #[case] ty: Ty) {
            let s = Statement::expression(
                Expression::integer(0, Span::default()),
                implicit,
                Span::default(),
            );

            let ty_info = s
                .solve(&mut Compiler::default(), &mut Scope::new())
                .unwrap()
                .get_ty_info()
                .clone();

            assert_eq!(ty_info.ty, ty);
            assert_eq!(ty_info.return_ty, None);
        }
    }
}
