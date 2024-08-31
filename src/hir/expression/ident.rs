use crate::stage::parse::ParseError;

use super::*;

use std::hash::Hash;

ast_node! {
    Ident<M> {
        binding: M::IdentIdentifier,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Ident<M> {
    fn register(parser: &mut Parser) {
        parser.register_prefix_test::<Expression<UntypedAstMetadata>>(
            |token| matches!(token, Token::Ident(_)),
            |_, compiler, lexer| {
                let (value, span) = match lexer.next_spanned().unwrap() {
                    (Token::Ident(value), span) => (value, span),
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Ident(String::new())),
                            found: Box::new(token),
                            reason: "expected ident".to_string(),
                        });
                    }
                };

                let binding = compiler.symbols.get_or_intern(value);

                Ok(Expression::Ident(Ident {
                    binding,
                    span,
                    ty_info: None,
                }))
            },
        );
    }
}

impl SolveType for Ident<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        _compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        let (binding, ty) = state
            .resolve(self.binding)
            .ok_or(TyError::SymbolNotFound(self.binding))?;

        Ok(Ident {
            ty_info: TyInfo {
                ty,
                return_ty: None,
            },
            binding,
            span: self.span,
        })
    }
}

impl<M: AstMetadata<IdentIdentifier: Hash>> Hash for Ident<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.binding.hash(state);
    }
}

impl<M: AstMetadata<IdentIdentifier: PartialEq>> PartialEq for Ident<M>
where
    M::IdentIdentifier: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding
    }
}

impl<M: AstMetadata> Eq for Ident<M> where M::IdentIdentifier: Eq {}

#[cfg(test)]
mod test_ident {
    use string_interner::Symbol;

    use super::*;

    mod parse {
        use crate::stage::parse::{Lexer, Precedence};

        use super::*;

        #[test]
        fn success() {
            let mut parser = Parser::new();

            Ident::<UntypedAstMetadata>::register(&mut parser);

            let mut compiler = Compiler::default();

            let ident: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut compiler,
                    &mut Lexer::from("someident"),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Ident(ident) = ident else {
                panic!("expected ident to be parsed");
            };

            assert_eq!(
                compiler.symbols.resolve(ident.binding).unwrap(),
                "someident"
            );
        }
    }

    mod ty {
        use super::*;

        #[test]
        fn ident_present() {
            // Set up a reference symbol
            let symbol = Symbol::try_from_usize(0).unwrap();

            // Create a scope and add the symbol to it
            let mut scope = Scope::new();
            scope.register(symbol, Ty::Int);

            let i = Ident::new(symbol, Span::default(), Default::default());

            // Run the type solve
            let ty_info = i
                .solve(&mut Compiler::default(), &mut scope)
                .unwrap()
                .ty_info;

            assert_eq!(ty_info.ty, Ty::Int);
            assert_eq!(ty_info.return_ty, None);
        }

        #[test]
        fn ident_infer_missing() {
            let i = Ident::new(
                Symbol::try_from_usize(0).unwrap(),
                Span::default(),
                Default::default(),
            );

            let result = i.solve(&mut Compiler::default(), &mut Scope::new());

            assert!(result.is_err());
        }
    }
}
