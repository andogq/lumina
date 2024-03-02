use crate::{
    core::ast::{InfixExpression, InfixOperatorToken},
    return_value,
    runtime::{
        object::{BooleanObject, IntegerObject, NullObject, Object, StringObject},
        Environment,
    },
    stages::interpreter::expression::interpret_expression,
    stages::interpreter::runtime::{error::Error, return_value::Return},
};

pub fn interpret_infix(env: &mut Environment, infix: InfixExpression) -> Return<Object> {
    use InfixOperatorToken::*;

    let left = return_value!(interpret_expression(env, *infix.left));
    let right = return_value!(interpret_expression(env, *infix.right));

    Return::Implicit(match (&infix.operator_token, left, right) {
        (token, Object::Integer(left), Object::Integer(right)) => {
            let left = left.value;
            let right = right.value;

            match token {
                Plus(_) | Minus(_) | Asterisk(_) | Slash(_) => Object::Integer(IntegerObject {
                    value: match token {
                        Plus(_) => left + right,
                        Minus(_) => left - right,
                        Asterisk(_) => left * right,
                        Slash(_) => left / right,
                        _ => unreachable!(),
                    },
                }),
                LeftAngle(_) | RightAngle(_) | Eq(_) | NotEq(_) => Object::Boolean(BooleanObject {
                    value: match token {
                        LeftAngle(_) => left < right,
                        RightAngle(_) => left > right,
                        Eq(_) => left == right,
                        NotEq(_) => left != right,
                        _ => unreachable!(),
                    },
                }),
            }
        }
        (token, Object::Boolean(left), Object::Boolean(right)) => {
            let left = left.value;
            let right = right.value;

            Object::Boolean(BooleanObject {
                value: match token {
                    LeftAngle(_) => left < right,
                    RightAngle(_) => left > right,
                    Eq(_) => left == right,
                    NotEq(_) => left != right,
                    _ => return Return::Implicit(Object::Null(NullObject)),
                },
            })
        }
        (Plus(_), Object::String(left), Object::String(right)) => Object::String(StringObject {
            value: left.value + &right.value,
        }),

        // If hasn't already been evaluated, left and right aren't equal
        (Eq(_), _, _) => Object::Boolean(BooleanObject { value: false }),
        (NotEq(_), _, _) => Object::Boolean(BooleanObject { value: true }),

        _ => return Error::throw("insupported infix operation"),
    })
}
