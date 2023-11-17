// lambda:
//     '('identifier*')''=>' expression

use std::string;

use crate::lexer::{self, TokenKind};

use super::{Expression, Parse, ParseError};

#[derive(Debug, Clone , PartialEq)]
pub struct Lambda {
    args: Vec<string::String>,
    expression: Box<Expression>
}

impl Lambda {
    fn new(args: Vec<string::String>, expression: Box<Expression>) -> Self {
        return Lambda { args, expression};
    }
}

impl Parse for Lambda {
    fn parse( tokens: &mut Vec<crate::lexer::Token>) -> Result<Self, super::ParseError>
        where
            Self: Sized {

                match tokens.pop() {
                    Some(token) => {
                        if token.kind != lexer::TokenKind::Operator || token.value != "(" {
                            return Err(ParseError::new("Expected '('", token.line, token.column));
                        }
                    } None => {
                        return Err(ParseError::new("Expected token but found none", 0, 0));
                    }
                }

                let mut args = Vec::new();
                loop {
                    match tokens.last() {
                        Some(token) => {
                            if token.kind == TokenKind::Operator && token.value == ")" {
                                tokens.pop();
                                break;
                            }
                        }
                        None => {
                            return Err(ParseError::new("Expected ')'", 0, 0));
                        }
                    }
                    match tokens.pop() {
                        Some(token) => {
                            if token.kind != TokenKind::Symbol {
                                return Err(ParseError::new("Expected identifier", token.line, token.column));
                            }
                            args.push(token.value);
                        }
                        None => {
                            return Err(ParseError::new("Expected identifier", 0, 0));
                        }
                    }
                }
                
                match tokens.pop() {
                    Some(token) => {
                        if token.kind != TokenKind::Operator || token.value != "=>" {
                            return Err(ParseError::new("Expected '=>'", token.line, token.column));
                        }
                    }
                    None => {
                        return Err(ParseError::new("Expected '=>'", 0, 0));
                    }
                }

                let expression = Expression::parse(tokens)?;

                return Ok(Lambda::new(args, Box::new(expression)));
    }
}

#[cfg(test)]
mod test_lambda {
    use crate::{lexer, parse::{Parse, Expression}};

    #[test]
    fn test_lambda_returns_error_when_no_open_paren() {
        let mut tokens = vec!(
            lexer::Token::new(lexer::TokenKind::Operator, ")".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "x".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, "=>".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 0, 0),
        ).into_iter().rev().collect();
        let lambda = super::Lambda::parse(&mut tokens);
        assert_eq!(lambda, Err(super::ParseError::new("Expected '('", 0, 0)));
    }

    #[test]
    fn test_lambda_returns_error_when_no_close_paren() {
        let mut tokens = vec!(
            lexer::Token::new(lexer::TokenKind::Operator, "(".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "x".to_string(), 0, 0),
        ).into_iter().rev().collect();
        let lambda = super::Lambda::parse(&mut tokens);
        assert_eq!(lambda, Err(super::ParseError::new("Expected ')'", 0, 0)));
    }

    #[test]
    fn test_lambda_returns_error_when_no_arrow() {
        let mut tokens = vec!(
            lexer::Token::new(lexer::TokenKind::Operator, "(".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "x".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, ")".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 0, 0),
        ).into_iter().rev().collect();
        let lambda = super::Lambda::parse(&mut tokens);
        assert_eq!(lambda, Err(super::ParseError::new("Expected '=>'", 0, 0)));
    }

    #[test]
    fn test_lambda_with_one_argument() {
        let mut tokens = vec!(
            lexer::Token::new(lexer::TokenKind::Operator, "(".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "x".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, ")".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, "=>".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 0, 0),
        ).into_iter().rev().collect();
        let lambda = super::Lambda::parse(&mut tokens);
        assert!(lambda.is_ok());
        assert_eq!(lambda.unwrap(), super::Lambda::new(vec!("x".to_string()), Box::new(Expression::Number(1))));
    }

    #[test]
    fn test_lambda_with_two_arguments() {
        let mut tokens = vec!(
            lexer::Token::new(lexer::TokenKind::Operator, "(".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "x".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "y".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, ")".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, "=>".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 0, 0),
        ).into_iter().rev().collect();

        let lambda = super::Lambda::parse(&mut tokens);
        assert!(lambda.is_ok());
        assert_eq!(lambda.unwrap(), super::Lambda::new(vec!("x".to_string(), "y".to_string()), Box::new(Expression::Number(1))));
    }
}