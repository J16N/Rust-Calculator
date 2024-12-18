use wasm_bindgen::JsError;

pub(super) enum Token {
    Number(f64),
    Operator(char),
    Parenthesis(char),
    Eof,
}

impl From<Token> for String {
    fn from(token: Token) -> String {
        match token {
            Token::Number(number) => number.to_string(),
            Token::Operator(operator) => operator.to_string(),
            Token::Parenthesis(parenthesis) => parenthesis.to_string(),
            Token::Eof => "EOF".to_string(),
        }
    }
}

pub(super) fn tokenize(expression: String) -> Result<Vec<Token>, JsError> {
    let mut tokens = Vec::new();
    let mut chars_iters = expression.chars().peekable();
    while let Some(char) = chars_iters.next() {
        match char {
            '0'..='9' => {
                let mut number = String::new();
                number.push(char);

                while let Some(&next_char) = chars_iters.peek() {
                    match next_char {
                        '0'..='9' => {
                            number.push(next_char);
                            chars_iters.next();
                        }
                        '.' => {
                            number.push(next_char);
                            chars_iters.next();
                        }
                        _ => break,
                    }
                }

                tokens.push(Token::Number(number.parse().unwrap()));
            }
            '+' | '-' | '*' | '/' => tokens.push(Token::Operator(char)),
            '(' | ')' => tokens.push(Token::Parenthesis(char)),
            _ => return Err(JsError::new(&format!("Invalid character: {}", char))),
        }
    }
    tokens.push(Token::Eof);
    Ok(tokens)
}
