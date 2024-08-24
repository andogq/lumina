#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    Uint,
    Boolean,
    Unit,
    Never,
}

impl Ty {
    pub fn check(&self, other: &Ty) -> bool {
        match (self, other) {
            (lhs, rhs) if lhs == rhs => true,
            (Ty::Never, _) | (_, Ty::Never) => true,
            _ => false,
        }
    }
}
