#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
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
