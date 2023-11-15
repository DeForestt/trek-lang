// block:
//  '{' expression* '}'

use crate::lexer;

use super::{Expression, Parse, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub expressions: Vec<Expression>,
}

impl Block {
    pub fn new(expressions: Vec<Expression>) -> Self {
        Block {
            expressions,
        }
    }
}

impl Parse for Block {
    fn parse( tokens: &mut Vec<lexer::Token>) -> Result<Self, ParseError>
        where
            Self: Sized {
        match tokens.last() {
            Some(token) => {
                if token.kind != lexer::TokenKind::Operator || token.value != "{" {
                    return Err(ParseError::new("Expected '{'", token.line, token.column));
                }
                tokens.pop();

            }
            None => {
                return Err(ParseError::new("Expected '{'", 0, 0));
            }
        }

        let mut expressions = Vec::new();
        loop {
            match tokens.last() {
                Some(token) => {
                    if token.kind == lexer::TokenKind::Operator && token.value == "}" {
                        tokens.pop();
                        break;
                    }
                }
                None => {
                    return Err(ParseError::new("Expected '}'", 0, 0));
                }
            }
            expressions.push(Expression::parse(tokens)?);
        }

        Ok(Block::new(expressions))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_error_when_no_open_brace() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Operator, "}".to_string(), 0, 0),
        ].into_iter().rev().collect();
        let block = Block::parse(&mut tokens);
        assert!(block.is_err());
        assert_eq!(block.unwrap_err(), ParseError::new("Expected '{'", 0, 0));
    }

    #[test]
    fn returns_error_when_no_close_brace() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Operator, "{".to_string(), 0, 0),
        ].into_iter().rev().collect();
        let block = Block::parse(&mut tokens);
        assert!(block.is_err());
        assert_eq!(block.unwrap_err(), ParseError::new("Expected '}'", 0, 0));
    }

    #[test]
    fn parses_an_empty_block() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Operator, "{".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, "}".to_string(), 0, 0),
        ].into_iter().rev().collect();
        let block = Block::parse(&mut tokens);
        assert!(block.is_ok());
        assert_eq!(block.unwrap(), Block::new(vec![]));
    }

    #[test]
    fn parses_with_expressions() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Operator, "{".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Operator, "}".to_string(), 0, 0),
        ].into_iter().rev().collect();
        let block = Block::parse(&mut tokens);
        assert!(block.is_ok());
        assert_eq!(block.unwrap(), Block::new(vec![Expression::Number(1)]));
    }
}