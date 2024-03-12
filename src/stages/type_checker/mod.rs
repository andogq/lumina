use std::collections::HashMap;

use crate::core::ast::{Expression, InfixExpression, PrefixExpression};

/// Primitive types available in the language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Boolean,
    Integer,
    String,
}

/// A map between variable names and their associated type
type Context = HashMap<String, Type>;

fn type_check_expression(e: &Expression, ctx: &Context) -> Result<Type, &'static str> {
    match e {
        Expression::Identifier(ident) => ctx
            .get(&ident.value)
            .cloned()
            .ok_or("expected ident in context"),
        Expression::Integer(_) => Ok(Type::Integer),
        Expression::String(_) => Ok(Type::String),
        Expression::Boolean(_) => Ok(Type::Boolean),
        Expression::Prefix(PrefixExpression { right, .. }) => {
            // WARN: Type may change depending on prefix operation
            type_check_expression(&right, ctx)
        }
        Expression::Infix(InfixExpression { left, right, .. }) => {
            let left = type_check_expression(&left, ctx)?;
            let right = type_check_expression(&right, ctx)?;

            // Make sure left and right are the same
            // WARN: This mightn't always be the case
            if left == right {
                Ok(left)
            } else {
                Err("expected left and right side of infix operation to be identical")
            }
        }
        Expression::Block(_) => todo!(),
        Expression::If(_) => todo!(),
        Expression::Function(_) => todo!(),
        Expression::Call(_) => todo!(),
    }
}

#[cfg(test)]
mod test {
    use crate::core::ast::{Identifier, InfixExpression, InfixOperatorToken};

    use super::*;

    #[test]
    fn simple_expression() {
        // x + y
        let e = Expression::Infix(InfixExpression {
            left: Box::new(Expression::Identifier(Identifier {
                value: "x".to_string(),
                ident_token: Default::default(),
            })),
            operator: "+".to_string(),
            right: Box::new(Expression::Identifier(Identifier {
                value: "y".to_string(),
                ident_token: Default::default(),
            })),
            operator_token: InfixOperatorToken::Plus(Default::default()),
        });

        let ctx = Context::from_iter([
            ("x".to_string(), Type::Integer),
            ("y".to_string(), Type::Integer),
        ]);

        assert_eq!(type_check_expression(&e, &ctx).unwrap(), Type::Integer);
    }
}
