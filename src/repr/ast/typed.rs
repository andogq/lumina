use crate::{generate_ast, repr::ty::Ty, stage::lower_ir::FunctionIdx};

#[derive(Clone, Debug)]
pub struct TyInfo {
    pub ty: Ty,
    pub return_ty: Option<Ty>,
}

generate_ast! {
    TyInfo: TyInfo,
    // TODO: Something else for this
    FnIdentifier: FunctionIdx
}
