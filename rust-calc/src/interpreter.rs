use wasm_bindgen::JsError;

use crate::parser::Expressions;
use crate::scanner::Token;

pub(super) fn interpret(expr: Expressions) -> Result<String, JsError> {
    visit(expr)
        .map(|token| token.into())
        .map_err(|error| JsError::new(&error))
}

fn visit(expr: Expressions) -> Result<Token, String> {
    match expr {
        Expressions::Binary {
            operator,
            left,
            right,
        } => {
            let left = visit(*left)?;
            let right = visit(*right)?;
            match (&left, &right) {
                (Token::Number(left), Token::Number(right)) => match operator {
                    Token::Operator('+') => Ok(Token::Number(left + right)),
                    Token::Operator('-') => Ok(Token::Number(left - right)),
                    Token::Operator('*') => Ok(Token::Number(left * right)),
                    Token::Operator('/') => {
                        if right == &0.0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Token::Number(left / right))
                    }
                    _ => {
                        let op: String = operator.into();
                        Err(format!("Invalid operator: {}", op))
                    }
                },
                _ => {
                    let left: String = left.into();
                    let right: String = right.into();
                    Err(format!("Invalid operands: {} and {}", left, right))
                }
            }
        }
        Expressions::Unary { operator, operand } => {
            let operand = visit(*operand)?;
            if let Token::Number(operand) = operand {
                match operator {
                    Token::Operator('-') => Ok(Token::Number(-operand)),
                    Token::Operator('+') => Ok(Token::Number(operand)),
                    _ => {
                        let op: String = operator.into();
                        Err(format!("Invalid operator: {}", op))
                    }
                }
            } else {
                let operand: String = operand.into();
                Err(format!("Invalid operand: {}", operand))
            }
        }
        Expressions::Grouping { expression } => {
            let result = visit(*expression)?;
            Ok(result)
        }
        Expressions::Literal { value } => Ok(value),
    }
}
