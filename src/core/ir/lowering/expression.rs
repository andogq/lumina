use crate::core::ir::*;

/// Lower the provided expression into the provided basic block, ensuring that the result of the
/// expression is stored into the target. The resulting basic block of the control flow must be
/// returned, even if it is unchanged.
pub fn lower_expression(
    _ctx: &Context,
    bb: BasicBlockBuilder,
    expr: ast::Expression,
    target: Local,
) -> BasicBlockBuilder {
    match expr {
        ast::Expression::Infix(_) => todo!(),
        ast::Expression::Integer(i) => bb.statement(Statement::Assign(
            target,
            RValue::Scalar(Scalar::int(i.value)),
        )),
        ast::Expression::Boolean(_) => todo!(),
        ast::Expression::Ident(_) => todo!(),
        ast::Expression::Block(_) => todo!(),
        ast::Expression::If(_) => todo!(),
    }
}

