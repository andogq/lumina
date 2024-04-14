use std::{marker::PhantomData, num::NonZeroU8};

use super::index::Index;

/// Declaration of a local, which represents some place in memory, most likely on the stack. Will
/// eventually include type information.
#[derive(Clone)]
pub struct LocalDecl;
pub type Local = Index<LocalDecl>;

/// The local that will hold the return value is always index `0`.
pub const RETURN_LOCAL: Local = Index::<LocalDecl>(0, PhantomData);

/// A scalar value is a constant value used within the compiler. Different data types (such as
/// integers and booleans) are 'normalised' into this representation to assist with lowering into
/// LLVM IR.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scalar {
    /// The value of this scalar.
    data: u64,

    /// Number of bytes the scalar value takes up.
    size: NonZeroU8,
}

impl Scalar {
    pub fn int(value: i64) -> Self {
        Self {
            data: value as u64,
            size: NonZeroU8::new(8).unwrap(),
        }
    }
}

// HACK: Is this really necessary?
impl PartialEq<&Scalar> for Scalar {
    fn eq(&self, other: &&Scalar) -> bool {
        self.eq(*other)
    }
}

// HACK: Is this also necessary?
impl PartialEq<Scalar> for &Scalar {
    fn eq(&self, other: &Scalar) -> bool {
        (*self).eq(other)
    }
}

/// Representation of the 'right hand' value in an expression. This can include some constant that
/// is loaded by the compiler, but could also include some other local.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RValue {
    Scalar(Scalar),
}
