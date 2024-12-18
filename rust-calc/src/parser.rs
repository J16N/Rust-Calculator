use wasm_bindgen::JsError;

use crate::scanner::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

pub(super) enum Expressions {
    Binary {
        operator: Token,
        left: Box<Expressions>,
        right: Box<Expressions>,
    },
    Unary {
        operator: Token,
        operand: Box<Expressions>,
    },
    Grouping {
        expression: Box<Expressions>,
    },
    Literal {
        value: Token,
    },
}

pub(super) fn parse(tokens: Vec<Token>) -> Result<Expressions, JsError> {
    let mut tokens = tokens.into_iter().peekable();
    let expression = match term(&mut tokens) {
        Ok(expression) => expression,
        Err(error) => return Err(JsError::new(&error)),
    };
    Ok(expression)
}

fn term(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Expressions, String> {
    let mut left = factor(tokens)?;

    while let Some(token) =
        tokens.next_if(|x| matches!(x, Token::Operator('-') | Token::Operator('+')))
    {
        let right = factor(tokens)?;
        left = Expressions::Binary {
            operator: token,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn factor(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Expressions, String> {
    let mut left = unary(tokens)?;

    while let Some(token) =
        tokens.next_if(|x| matches!(x, Token::Operator('*') | Token::Operator('/')))
    {
        let right = unary(tokens)?;
        left = Expressions::Binary {
            operator: token,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn unary(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Expressions, String> {
    if let Some(token) =
        tokens.next_if(|x| matches!(x, Token::Operator('+') | Token::Operator('-')))
    {
        let operand = unary(tokens)?;
        return Ok(Expressions::Unary {
            operator: token,
            operand: Box::new(operand),
        });
    }
    primary(tokens)
}

fn primary(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Expressions, String> {
    match tokens.peek() {
        Some(Token::Number(_)) => {
            let token = tokens.next().unwrap();
            Ok(Expressions::Literal { value: token })
        }

        Some(Token::Parenthesis('(')) => {
            tokens.next();
            let expression = term(tokens)?;
            match tokens.next() {
                Some(Token::Parenthesis(')')) => Ok(Expressions::Grouping {
                    expression: Box::new(expression),
                }),
                _ => Err("Expected ')'".to_string()),
            }
        }

        _ => Err("Expected expression".to_string()),
    }
}
