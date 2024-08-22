use crate::{compiler::Symbol, generate_ast, repr::ty::Ty, util::span::Span};

use super::base::AstMetadata;

#[derive(Debug)]
pub struct UntypedAstMetadata;
impl AstMetadata for UntypedAstMetadata {
    type FnIdentifier = Symbol;
    type IdentIdentifier = Symbol;
    type TyInfo = Option<Ty>;
    type Span = Span;
}

generate_ast!(UntypedAstMetadata);
