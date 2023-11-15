use crate::lexer;
mod definition;
mod block;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    message: String,
    line: usize,
    column: usize,
}

impl ParseError {
    fn new(message: &str, line: usize, column: usize) -> Self {
        ParseError {
            message: message.to_string(),
            line,
            column,
        }
    }
}

trait Parse {
    fn parse( tokens: &mut Vec<lexer::Token>) -> Result<Self, ParseError>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Definition(definition::Definition),
    Number(i32),
    Block(block::Block),
}

impl Expression {
    pub fn parse(tokens: &mut Vec<lexer::Token>) -> Result<Self, ParseError> {
        match tokens.last() {
            Some(token) => {
                match token.kind {
                    lexer::TokenKind::Number => {
                        let value = match tokens.pop() {
                            Some(token) => {
                                match token.value.parse::<i32>() {
                                    Ok(value) => value,
                                    Err(_) => {
                                        return Err(ParseError::new("Expected number", token.line, token.column));
                                    }
                                }
                            }
                            None => {
                                return Err(ParseError::new("Expected number", 0, 0));
                            }
                        };
                        return Ok(Expression::Number(value));
                    }
                    lexer::TokenKind::Operator => {
                        match token.value.as_str() {
                            "{" => {
                                return Ok(Expression::Block(block::Block::parse(tokens)?));
                            }
                            _ => {
                                return Err(ParseError::new("Unknown Operator", token.line, token.column));
                            }
                        }
                    }
                    lexer::TokenKind::Symbol => {
                        match token.value.as_str() {
                            "let" => {
                                return Ok(Expression::Definition(definition::Definition::parse(tokens)?));
                            }
                            _ => {
                                return Err(ParseError::new("Unknown KW", token.line, token.column));
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError::new("Expected number or 'let' keyword", token.line, token.column));
                    }
                }
            }
            None => {
                return Err(ParseError::new("Unknown Parser Error", 0, 0));
            }
        }
    }
}