mod symbol;

pub use self::symbol::*;

/// Compile context used throughout the entire process
#[derive(Default)]
pub struct Ctx {
    pub symbols: SymbolMap,
}
