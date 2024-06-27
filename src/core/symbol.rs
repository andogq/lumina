use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Clone, Default)]
pub struct SymbolMap {
    next: usize,
    map: HashMap<String, Symbol>,
    names: HashMap<Symbol, String>,
}

impl SymbolMap {
    pub fn new() -> Self {
        Self {
            next: 0,
            map: HashMap::new(),
            names: HashMap::new(),
        }
    }

    pub fn get(&mut self, symbol: impl ToString) -> Symbol {
        *self.map.entry(symbol.to_string()).or_insert_with(|| {
            let id = self.next;
            self.next += 1;

            let s = Symbol(id);
            self.names.insert(s, symbol.to_string());

            s
        })
    }

    pub fn name(&self, symbol: Symbol) -> Option<String> {
        self.names.get(&symbol).cloned()
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Default)]
pub struct Symbol(usize);
impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
