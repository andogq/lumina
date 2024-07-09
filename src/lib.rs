use core::ctx::{Ctx, SymbolMap, SymbolMapTrait};

use repr::token::Token;
use stage::{
    lex::Lexer,
    parse::{ParseCtxTrait, TokenGenerator},
};
use util::source::Source;

pub mod core;
pub mod repr;
pub mod stage;
pub mod util;

// TODO: Move this to a better place
pub struct ParseCtx {
    symbols: SymbolMap,
    lexer: Lexer,
}

impl ParseCtx {
    pub fn new(source: Source) -> Self {
        Self {
            symbols: SymbolMap::new(),
            lexer: Lexer::new(source),
        }
    }
}

impl SymbolMapTrait for ParseCtx {
    fn intern(&mut self, s: impl AsRef<str>) -> core::ctx::Symbol {
        SymbolMapTrait::intern(&mut self.symbols, s)
    }

    fn get(&self, s: core::ctx::Symbol) -> String {
        SymbolMapTrait::get(&self.symbols, s)
    }

    fn dump_symbols(&self) -> SymbolMap {
        SymbolMapTrait::dump_symbols(&self.symbols)
    }
}

impl TokenGenerator for ParseCtx {
    fn peek_token(&mut self) -> Token {
        TokenGenerator::peek_token(&mut self.lexer)
    }

    fn next_token(&mut self) -> Token {
        TokenGenerator::next_token(&mut self.lexer)
    }
}

impl ParseCtxTrait for ParseCtx {}

// TODO: burn
impl From<ParseCtx> for Ctx {
    fn from(value: ParseCtx) -> Self {
        Self {
            symbols: value.symbols,
        }
    }
}
