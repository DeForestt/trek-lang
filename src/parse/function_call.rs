// function_call:
//     identifier '('(expression (, expression)*)?')'

use crate::lexer::{Token, TokenKind};

use super::{Parse, Expression, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    name: String,
    args: Vec<Expression>,
}

impl FunctionCall {
    fn new(name: String, args: Vec<Expression>) -> Self {
        FunctionCall { name, args }
    }
}

impl Parse for FunctionCall {
    fn parse( tokens: &mut Vec<Token>) -> Result<Self, ParseError>
        where
            Self: Sized {
        let name = match tokens.pop() {
            Some(token) => {
                if token.kind != crate::lexer::TokenKind::Symbol {
                    return Err(ParseError::new("Expected identifier", token.line, token.column));
                }
                token.value
            }
            None => {
                return Err(ParseError::new("Expected identifier", 0, 0));
            }
        };

        match tokens.pop() {
            Some(token) => {
                if token.kind != crate::lexer::TokenKind::Operator || token.value != "(" {
                    return Err(ParseError::new("Expected '('", token.line, token.column));
                }
            }
            None => {
                return Err(ParseError::new("Expected '('", 0, 0));
            }
        }

        match tokens.last() {
            Some(token) => {
                if token.kind == TokenKind::Operator && token.value == ")" {
                    tokens.pop();
                    return Ok(FunctionCall::new(name, Vec::new()));
                }

                if token.kind == TokenKind::Operator && token.value == "," {
                    return Err(ParseError::new("Expected expression", token.line, token.column));
                }

                let mut args = Vec::new();
                // push the first argument
                args.push(Expression::parse(tokens)?);
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
                            if token.kind != TokenKind::Operator || token.value != "," {
                                println!("{:?}", token);
                                return Err(ParseError::new("Expected ','", token.line, token.column));
                            }
                        }
                        None => {
                            return Err(ParseError::new("Expected ','", 0, 0));
                        }
                    }
                    args.push(Expression::parse(tokens)?);
                }
                return Ok(FunctionCall::new(name, args));
            }
            None => {
                return Err(ParseError::new("Expected ')'", 0, 0));
            }
        }
    }
}

#[cfg(test)]
mod test_function_call {
    use crate::{lexer, parse::{Parse, Expression}};
    use super::*;

    #[test]
    fn test_function_call_with_one_arg() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "foo".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, "(".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "bar".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, ")".to_string(), 0, 0),
        ].into_iter().rev().collect();
        let function_call = FunctionCall::parse(&mut tokens).unwrap();
        assert_eq!(function_call, FunctionCall::new("foo".to_string(), vec![Expression::Identifier("bar".to_string())]));
    }

    #[test]
    fn test_function_call_with_two_args() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "foo".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, "(".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "bar".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, ",".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Symbol, "baz".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, ")".to_string(), 0, 0),
        ].into_iter().rev().collect();
        let function_call = FunctionCall::parse(&mut tokens).unwrap();
        assert_eq!(function_call, FunctionCall::new("foo".to_string(), vec![Expression::Identifier("bar".to_string()), Expression::Identifier("baz".to_string())]));
    }
}
