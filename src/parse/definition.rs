use super::{Parse, ParseError, Expression};
use crate::lexer;


// let: 'let' identifier '=' expression
#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    pub identifier: String,
    pub expression: Box<Expression>,
}

impl Definition {
    pub fn new(identifier: String, expression: Box<Expression>) -> Self {
        Definition {
            identifier,
            expression,
        }
    }
}

impl Parse for Definition {
    fn parse(tokens: &mut Vec<lexer::Token>) -> Result<Self, ParseError>
    where
        Self: Sized {
            match tokens.last() {
                Some(token) => {
                    if token.kind != lexer::TokenKind::Symbol || token.value != "let" {
                        println!("{:?}", token);
                        return Err(ParseError::new("Expected 'let' keyword", token.line, token.column));
                    }
                    tokens.pop();
                }
                None => {
                    return Err(ParseError::new("Expected 'let' keyword", 0, 0));
                }
            }

            let identifier = match tokens.pop() {
                Some(token) => {
                    if token.kind != lexer::TokenKind::Symbol {
                        return Err(ParseError::new("Expected identifier", token.line, token.column));
                    }
                    token.value
                }
                None => {
                    return Err(ParseError::new("Expected identifier", 0, 0));
                }
            };

            match tokens.last() {
                Some(token) => {
                    if token.kind != lexer::TokenKind::Operator || token.value != "=" {
                        return Err(ParseError::new("Expected '='", token.line, token.column));
                    }
                    tokens.pop();
                }
                None => {
                    return Err(ParseError::new("Expected '='", 0, 0));
                }
            }

            let expression = Expression::parse(tokens)?;

            return Ok(Definition::new(identifier, Box::new(expression)));

        }
}

#[cfg(test)]
mod test_definition {
    use crate::{lexer, parse::Parse};
    use super::{Definition, Expression};

    #[test]
    fn returns_error_if_not_let() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "do".to_string(), 1, 1),
            lexer::Token::new(lexer::TokenKind::Symbol, "something".to_string(), 1, 3),
            lexer::Token::new(lexer::TokenKind::Operator, "=".to_string(), 1, 13),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 1, 15),
        ].into_iter().rev().collect();

        let result = Definition::parse(&mut tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Expected 'let' keyword");
    }

    #[test]
    fn returns_error_if_no_identifier() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "let".to_string(), 1, 1),
            lexer::Token::new(lexer::TokenKind::Operator, "=".to_string(), 1, 5),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 1, 7),
        ].into_iter().rev().collect();

        let result = Definition::parse(&mut tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Expected identifier");
    }

    #[test]
    fn returns_error_if_no_eq() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "let".to_string(), 1, 1),
            lexer::Token::new(lexer::TokenKind::Symbol, "x".to_string(), 1, 5),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 1, 7),
        ].into_iter().rev().collect();

        let result = Definition::parse(&mut tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Expected '='");
    }

    #[test]
    fn returns_with_a_parsed_number() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "let".to_string(), 1, 1),
            lexer::Token::new(lexer::TokenKind::Symbol, "x".to_string(), 1, 5),
            lexer::Token::new(lexer::TokenKind::Operator, "=".to_string(), 1, 7),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 1, 9),
        ].into_iter().rev().collect();

        let result = Definition::parse(&mut tokens);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Definition::new("x".to_string(), Box::new(Expression::Number(1))));
        assert!(tokens.is_empty());
    }
}