use crate::{
    generate_ast, repr::ty::Ty, stage::lower_ir::FunctionIdx,
    util::symbol_map::interner_symbol_map::Symbol,
};

#[derive(Clone, Debug)]
pub struct TyInfo {
    pub ty: Ty,
    pub return_ty: Option<Ty>,
}

generate_ast! {
    TyInfo: TyInfo,
    // TODO: Something else for this
    FnIdentifier: Symbol
}
