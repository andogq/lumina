use crate::{
    codegen::ir::{lowering::lower_expression, value::*, *},
    core::ast,
};

use self::function::Function;

/// Lower the provided function AST node.
pub fn lower_function(ctx: &Context, node: ast::Function) -> Function {
    let entry = ctx.basic_block();

    for s in node.body.statements {
        match s {
            ast::Statement::Return(s) => {
                return Function {
                    // Evaluate the expression into the return local
                    entry: lower_expression(ctx, entry, s.value, RETURN_LOCAL)
                        // Trigger the return terminator
                        .t_return(),

                    name: node.name,
                };
            }
            ast::Statement::Let(_) => todo!(),
            ast::Statement::Expression(_) => todo!(),
        }
    }

    panic!("function body had zero statements")
}

#[cfg(test)]
mod test {
    use crate::core::ty::Ty;

    use super::*;

    #[test]
    fn return_integer() {
        /*
         * fn main() -> int {
         *     return 1;
         * }
         */
        let f = ast::Function {
            span: Default::default(),
            name: Default::default(),
            parameters: Vec::new(),
            return_ty: Ty::Int,
            body: ast::Block::new(&[ast::Statement::_return(ast::Expression::integer(1))]),
        };

        let (ir, ctx) = {
            let ctx = Context::new();

            (
                lower_function(&ctx, f),
                Rc::into_inner(ctx.0).unwrap().into_inner(),
            )
        };

        /*
         * entry: {
         *     Assign(0, RValue::Scalar(1))
         *     Return
         * }
         */
        let entry_bb = &ctx.basic_blocks[ir.entry];

        assert_eq!(entry_bb.statements.len(), 1);
        assert!(matches!(
            entry_bb.statements[0],
            Statement::Assign(_, RValue::Scalar(_))
        ));

        if let Statement::Assign(target, RValue::Scalar(ref value)) = entry_bb.statements[0] {
            assert_eq!(target, RETURN_LOCAL);
            assert_eq!(value, Scalar::int(1));
        }
    }
}
