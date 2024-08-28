use super::*;

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

    pub fn expression(expression: Expression<M>, implicit_return: bool, span: M::Span) -> Self {
        Self::ExpressionStatement(ExpressionStatement::new(
            expression,
            implicit_return,
            span,
            M::TyInfo::default(),
        ))
    }
}

ast_node! {
    Return<M> {
        value: Expression<M>,
        span,
        ty_info,
    }
}

impl SolveType for Return<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        let value = self.value.solve(compiler, state)?;

        Ok(Return {
            ty_info: TyInfo::try_from((
                Ty::Never,
                [
                    Some(value.get_ty_info().ty.clone()),
                    value.get_ty_info().return_ty.clone(),
                ],
            ))?,
            value,
            span: self.span,
        })
    }
}

ast_node! {
    Let<M> {
        binding: M::IdentIdentifier,
        value: Expression<M>,
        span,
        ty_info,
    }
}

impl SolveType for Let<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        // Work out what the type of the value is
        let value = self.value.solve(compiler, state)?;

        // Make sure the value type matches what the statement was annotated with
        if let Some(ty) = self.ty_info {
            let value_ty = value.get_ty_info();
            if !ty.check(&value_ty.ty) {
                return Err(TyError::Mismatch(ty, value_ty.ty.clone()));
            }
        }

        // Record the type
        let binding = state.register(self.binding, value.get_ty_info().ty.clone());

        Ok(Let {
            ty_info: TyInfo {
                ty: Ty::Unit,
                return_ty: value.get_ty_info().return_ty.clone(),
            },
            binding,
            value,
            span: self.span,
        })
    }
}

ast_node! {
    ExpressionStatement<M> {
        expression: Expression<M>,
        implicit_return: bool,
        span,
        ty_info,
    }
}

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

ast_node! {
    Break<M> {
        span,
        ty_info,
    }
}

impl SolveType for Break<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        _compiler: &mut Compiler,
        _state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        Ok(Break {
            ty_info: TyInfo {
                ty: Ty::Never,
                return_ty: None,
            },
            span: self.span,
        })
    }
}

ast_node! {
    Continue<TyInfo> {
        span,
        ty_info,
    }
}

impl SolveType for Continue<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        _compiler: &mut Compiler,
        _state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        Ok(Continue {
            ty_info: TyInfo {
                ty: Ty::Never,
                return_ty: None,
            },
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test_statement {
    use string_interner::Symbol;

    use super::*;

    #[test]
    fn return_statement() {
        // return 0;
        let s = Statement::_return(Expression::integer(0, Span::default()), Span::default());

        let ty_info = s
            .solve(&mut Compiler::default(), &mut Scope::new())
            .unwrap()
            .get_ty_info()
            .clone();

        assert_eq!(ty_info.ty, Ty::Never);
        assert_eq!(ty_info.return_ty, Some(Ty::Int));
    }

    #[test]
    fn let_statement() {
        // let a = 0;
        let s = Statement::_let(
            Symbol::try_from_usize(0).unwrap(),
            Expression::integer(0, Span::default()),
            Span::default(),
        );

        let mut scope = Scope::new();

        let ty_info = s
            .solve(&mut Compiler::default(), &mut scope)
            .unwrap()
            .get_ty_info()
            .clone();

        assert_eq!(ty_info.ty, Ty::Unit);
        assert_eq!(ty_info.return_ty, None);
        assert_eq!(
            scope.resolve(Symbol::try_from_usize(0).unwrap()).unwrap().1,
            Ty::Int
        );
    }

    #[test]
    fn infer_expression() {
        // 0;
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            false,
            Span::default(),
        );

        let ty_info = s
            .solve(&mut Compiler::default(), &mut Scope::new())
            .unwrap()
            .get_ty_info()
            .clone();

        assert_eq!(ty_info.ty, Ty::Unit);
        assert_eq!(ty_info.return_ty, None);
    }

    #[test]
    fn infer_expression_implicit() {
        // 0
        let s = Statement::expression(
            Expression::integer(0, Span::default()),
            true,
            Span::default(),
        );

        let ty_info = s
            .solve(&mut Compiler::default(), &mut Scope::new())
            .unwrap()
            .get_ty_info()
            .clone();

        assert_eq!(ty_info.ty, Ty::Int);
        assert_eq!(ty_info.return_ty, None);
    }
}
