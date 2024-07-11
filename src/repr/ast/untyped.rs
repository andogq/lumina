use crate::{ctx::Symbol, generate_ast, repr::ty::Ty};

generate_ast! {
    TyInfo: Option<Ty>,
    FnIdentifier: Symbol
}
