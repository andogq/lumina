use crate::{
    generate_ast,
    repr::{identifier::FunctionIdx, ty::Ty},
};

#[derive(Clone, Debug)]
pub struct TyInfo {
    pub ty: Ty,
    pub return_ty: Option<Ty>,
}

generate_ast! {
    TyInfo: TyInfo,
    FnIdentifier: FunctionIdx
}
