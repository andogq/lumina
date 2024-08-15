use crate::repr::{
    identifier::{FunctionIdx, ScopedBinding},
    ty::Ty,
};

pub trait LLVMCtx {
    /// Produce the original name for a scoped binding.
    fn get_scoped_binding_name(&self, function: &FunctionIdx, binding: &ScopedBinding) -> String;

    /// Produce the type for a scoped binding.
    fn get_scoped_binding_ty(&self, function: &FunctionIdx, binding: &ScopedBinding) -> Ty;

    /// Get the name for a function.
    fn get_function_name(&self, function: &FunctionIdx) -> String;
}
