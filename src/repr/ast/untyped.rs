use crate::{compiler::Symbol, generate_ast, repr::ty::Ty};

generate_ast! {
    TyInfo: Option<Ty>,
    FnIdentifier: Symbol,
    IdentIdentifier: Symbol
}
