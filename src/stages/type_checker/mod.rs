use std::collections::HashMap;

use crate::core::ast::{
    Block, Expression, InfixExpression, LetStatement, PrefixExpression, Program, ReturnStatement,
    Statement,
};

/// Primitive types available in the language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Boolean,
    Integer,
    String,
    Unit,
}

/// A map between variable names and their associated type
type Context = HashMap<String, Type>;

pub fn type_check_program(p: Program) -> Result<Context, &'static str> {
    // Set up a new type context
    let mut ctx = Context::new();

    for statement in p.statements {
        let ty = type_check_statement(&statement, &mut ctx)?;

        if ty != Type::Unit {
            return Err("cannot return a value from a program");
        }
    }

    Ok(ctx)
}

pub fn type_check_block(b: &Block, ctx: &mut Context) -> Result<Type, &'static str> {
    let mut ty = Type::Unit;

    for statement in &b.statements {
        ty = type_check_statement(&statement, ctx)?;
    }

    Ok(ty)
}

fn type_check_statement(s: &Statement, ctx: &mut Context) -> Result<Type, &'static str> {
    match s {
        Statement::Let(LetStatement { name, value, .. }) => {
            // Determine the type of the value
            let ty = type_check_expression(value, ctx)?;

            // Update the context so the variable has the correct type
            ctx.insert(name.value.clone(), ty);

            Ok(Type::Unit)
        }
        Statement::Return(ReturnStatement { value, .. }) => Ok(type_check_expression(value, ctx)?),
        Statement::Expression {
            expression,
            semicolon,
        } => {
            let ty = type_check_expression(expression, ctx)?;

            Ok(if *semicolon {
                // Implicit return
                ty
            } else {
                Type::Unit
            })
        }
    }
}

fn type_check_expression(e: &Expression, ctx: &mut Context) -> Result<Type, &'static str> {
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
        Expression::Block(block) => type_check_block(block, ctx),
        Expression::If(_) => todo!(),
        Expression::Function(_) => todo!(),
        Expression::Call(_) => todo!(),
    }
}

#[cfg(test)]
mod test {
    use crate::core::ast::{Identifier, InfixExpression, InfixOperatorToken, IntegerLiteral};

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

        let mut ctx = Context::from_iter([
            ("x".to_string(), Type::Integer),
            ("y".to_string(), Type::Integer),
        ]);

        assert_eq!(type_check_expression(&e, &mut ctx).unwrap(), Type::Integer);
    }

    #[test]
    fn simple_program() {
        let p = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    let_token: Default::default(),
                    name: Identifier {
                        value: "a".to_string(),
                        ident_token: Default::default(),
                    },
                    value: Expression::Integer(IntegerLiteral::new(5)),
                }),
                Statement::Let(LetStatement {
                    let_token: Default::default(),
                    name: Identifier {
                        value: "b".to_string(),
                        ident_token: Default::default(),
                    },
                    value: Expression::Integer(IntegerLiteral::new(10)),
                }),
                Statement::Let(LetStatement {
                    let_token: Default::default(),
                    name: Identifier {
                        value: "c".to_string(),
                        ident_token: Default::default(),
                    },
                    value: Expression::Infix(InfixExpression {
                        operator_token: InfixOperatorToken::Plus(Default::default()),
                        operator: "+".to_string(),
                        left: Box::new(Expression::Identifier(Identifier {
                            value: "a".to_string(),
                            ident_token: Default::default(),
                        })),
                        right: Box::new(Expression::Identifier(Identifier {
                            value: "b".to_string(),
                            ident_token: Default::default(),
                        })),
                    }),
                }),
            ],
        };

        let ctx = type_check_program(p).unwrap();

        assert_eq!(ctx.get("a").unwrap(), &Type::Integer);
        assert_eq!(ctx.get("b").unwrap(), &Type::Integer);
        assert_eq!(ctx.get("c").unwrap(), &Type::Integer);
    }
}
