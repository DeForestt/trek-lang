use crate::lexer;
mod definition;
mod block;
mod if_expr;
mod lambda;
mod function_call;
pub mod module;

pub fn parse_module(tokens: &mut Vec<lexer::Token>) -> Result<module::Module, ParseError> {
    module::Module::parse(tokens)
}

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
    Lambda(lambda::Lambda),
    IfExpr(if_expr::IfExpr),
    Identifier(String),
    FunctionCall(function_call::FunctionCall),
    StringLiteral(String),
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
                            "(" => {
                                return Ok(Expression::Lambda(lambda::Lambda::parse(tokens)?));
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
                            "if" => {
                                return Ok(Expression::IfExpr(if_expr::IfExpr::parse(tokens)?));
                            }
                            _ => {
                                if tokens.len() < 2 {
                                    match tokens.pop() {
                                        Some(token) => {
                                            return Ok(Expression::Identifier(token.value.clone()));
                                        }
                                        None => {
                                            return Err(ParseError::new("Unknown Parser Error", 0, 0));
                                        }
                                    }
                                }

                                match tokens[tokens.len() - 2].kind {
                                    lexer::TokenKind::Operator => {
                                        match tokens[tokens.len() - 2].value.as_str() {
                                            "(" => {
                                                return Ok(Expression::FunctionCall(function_call::FunctionCall::parse(tokens)?));
                                            }
                                            _ => {
                                                match tokens.pop() {
                                                    Some(token) => {
                                                        return Ok(Expression::Identifier(token.value.clone()));
                                                    }
                                                    None => {
                                                        return Err(ParseError::new("Unknown Parser Error", 0, 0));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        return Ok(Expression::Identifier(token.value.clone()));
                                    }
                                }
                            }
                        }
                    }
                    lexer::TokenKind::StringLiteral => {
                        let value = match tokens.pop() {
                            Some(token) => {
                                token.value
                            }
                            None => {
                                return Err(ParseError::new("Expected string literal", 0, 0));
                            }
                        };
                        return Ok(Expression::StringLiteral(value));
                    }
                    _ => {
                        return Err(ParseError::new("Unknown Token", token.line, token.column));
                    }
                }
            }
            None => {
                return Err(ParseError::new("Unknown Parser Error", 0, 0));
            }
        }
    }
}