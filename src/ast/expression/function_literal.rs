use std::iter::Peekable;

use crate::{
    ast::BlockStatement,
    parser::Node,
    token::{FunctionToken, Token},
};

use super::Identifier;

pub struct FunctionLiteral {
    pub fn_token: FunctionToken,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String> {
        let fn_token = tokens
            .next()
            .and_then(|token| {
                if let Token::Function(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected function token".to_string())?;

        let l_paren = tokens
            .next()
            .and_then(|token| {
                if let Token::LeftBrace(token) = token {
                    Some(token)
                } else {
                    None
                }
            })
            .ok_or_else(|| "expected opening parenthesis".to_string())?;

        let mut parameters = Vec::new();

        loop {
            let param = Identifier::parse(tokens)?;
            parameters.push(param);

            // Either comma or closing paren must follow
            match tokens
                .next()
                .ok_or_else(|| "token to follow function parameter".to_string())?
            {
                Token::Comma(_) => {
                    // Continue to parse the parameter list
                    continue;
                }
                Token::RightBrace(_) => {
                    // ENd of parameter list
                    break;
                }
                token => {
                    return Err(format!("encountered unexpected token whilst parsing function parameter list: {token:?}"));
                }
            }
        }

        let body = BlockStatement::parse(tokens)?;

        Ok(Self {
            fn_token,
            parameters,
            body,
        })
    }
}

impl ToString for FunctionLiteral {
    fn to_string(&self) -> String {
        format!(
            "fn({}) {}",
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.body.to_string()
        )
    }
}
