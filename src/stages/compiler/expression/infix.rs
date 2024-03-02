use crate::{
    ast::{InfixExpression, InfixOperatorToken},
    code::Instruction,
    stages::compiler::Compiler,
};

impl Compiler {
    pub(super) fn compile_infix(&mut self, infix: InfixExpression) -> Result<(), String> {
        // Ordering of operands is determined by the instruction
        let mut operands = [*infix.left, *infix.right];

        let instruction = match infix.operator_token {
            InfixOperatorToken::Plus(_) => Instruction::Add,
            InfixOperatorToken::Minus(_) => Instruction::Sub,
            InfixOperatorToken::Asterisk(_) => Instruction::Mul,
            InfixOperatorToken::Slash(_) => Instruction::Div,
            InfixOperatorToken::LeftAngle(_) => {
                // Re-use greater than for less than, meaning params must be reversed
                operands.reverse();

                Instruction::GreaterThan
            }
            InfixOperatorToken::RightAngle(_) => Instruction::GreaterThan,
            InfixOperatorToken::Eq(_) => Instruction::Equal,
            InfixOperatorToken::NotEq(_) => Instruction::NotEqual,
        };

        let [lhs, rhs] = operands;

        // Compile LHS
        self.compile_expression(lhs)?;

        // Compile RHS
        self.compile_expression(rhs)?;

        // Insert instruction
        self.instructions.push(instruction);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Expression, IntegerLiteral},
        runtime::object::{IntegerObject, Object},
        token::PlusToken,
    };

    use super::*;

    #[test]
    fn addition() {
        let mut compiler = Compiler::default();
        compiler
            .compile_infix(InfixExpression {
                operator_token: InfixOperatorToken::Plus(PlusToken::default()),
                operator: "+".to_string(),
                left: Box::new(Expression::Integer(IntegerLiteral::new(1))),
                right: Box::new(Expression::Integer(IntegerLiteral::new(2))),
            })
            .unwrap();

        assert!(matches!(
            compiler.constants[0..2],
            [
                Object::Integer(IntegerObject { value: 1 }),
                Object::Integer(IntegerObject { value: 2 })
            ]
        ));
        assert_eq!(
            compiler.instructions,
            [
                Instruction::Constant(0),
                Instruction::Constant(1),
                Instruction::Add,
            ]
        );
    }
}
