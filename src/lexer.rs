use std::string;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorType {
    UnexpectedCharacter(char),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    error_type: LexerErrorType,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number,
    Operator,
    Symbol,
    NewLine,
    StringLiteral,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: string::String,
    pub column: usize,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, value: string::String, column: usize, line: usize) -> Self {
        Token {
            kind: kind,
            value: value,
            column: column,
            line: line,
        }
    }
}

pub fn tokenize(input: string::String) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: usize = 1;
    let mut column: usize = 1;
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            value.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::new(TokenKind::Number, value, column, line));
            }
            '+' | '-' | '*' | '/' | '{' | '}'=> {
                tokens.push(Token::new(TokenKind::Operator, c.to_string(), column, line));
                chars.next();
            },
            '=' => {
                // first check if it's a single or double equal
                chars.next();
                match chars.peek() {
                    Some(&c) => {
                        if c == '=' {
                            tokens.push(Token::new(TokenKind::Operator, "==".to_string(), column, line));
                            column += 2;
                            chars.next();
                        } else if chars.peek() == Some(&'>') {
                            tokens.push(Token::new(TokenKind::Operator, "=>".to_string(), column, line));
                            column += 2;
                            chars.next();
                        } else {
                            column += 1;
                            tokens.push(Token::new(TokenKind::Operator, "=".to_string(), column, line));
                        }
                    },
                    None => {
                        tokens.push(Token::new(TokenKind::Operator, "=".to_string(), column, line));
                    }
                }
            },
            'a'..='z' | 'A'..='Z' => {
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' => {
                            value.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::new(TokenKind::Symbol, value, column, line));
            }
            '"' => {
                chars.next();
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        '"' => {
                            chars.next();
                            break;
                        }
                        _ => {
                            value.push(c);
                            chars.next();
                        }
                    }
                }
                tokens.push(Token::new(TokenKind::StringLiteral, value, column, line));
            }
            '\n' => {
                tokens.push(Token::new(TokenKind::NewLine, "\n".to_string(), column, line));
                chars.next();
                line += 1;
                column = 1;
            },
            '\'' => {
                let mut value = String::new();
                chars.next();
                while let Some(&c) = chars.peek() {
                    match c {
                        '\'' => {
                            chars.next();
                            break;
                        }
                        _ => {
                            value.push(c);
                            chars.next();
                        }
                    }
                }
                tokens.push(Token::new(TokenKind::StringLiteral, value, column, line));
            },
            ' ' => {
                chars.next();
                column += 1;
            }
            _ => {
                return Err(LexerError {
                    error_type: LexerErrorType::UnexpectedCharacter(c),
                    line: line,
                    column: column,
                })
            }
        }
    }

    Ok(tokens)
}