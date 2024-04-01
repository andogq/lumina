use super::Expression;

pub enum Statement {
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

pub struct ReturnStatement {
    pub value: Expression,
}

pub struct ExpressionStatement {
    pub expression: Expression,
}
