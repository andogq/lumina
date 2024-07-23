use index_vec::define_index_type;

define_index_type! {pub struct FunctionIdx = usize;}
define_index_type! {pub struct ScopeIdx = usize;}
define_index_type! {pub struct BindingIdx = usize;}

/// A binding within a specific scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScopedBinding(pub ScopeIdx, pub BindingIdx);
