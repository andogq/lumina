use crate::{generate_ast, repr::ty::Ty, util::symbol_map::interner_symbol_map::Symbol};

generate_ast! {
    TyInfo: Option<Ty>,
    FnIdentifier: Symbol
}
