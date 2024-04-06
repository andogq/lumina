use super::{Block, Expression};

#[derive(Debug)]
pub struct If {
    pub condition: Box<Expression>,
    pub success: Block,
    pub otherwise: Option<Block>,
}
