use crate::ast_node2;

use super::*;

ast_node2! {
    Program<M> {
        functions: Vec<Function<M>>,
        main: Function<M>,
        span,
    }
}
