use crate::ast_node;

use super::*;

ast_node! {
    Program<M> {
        functions: Vec<Function<M>>,
        main: Function<M>,
        span,
    }
}
