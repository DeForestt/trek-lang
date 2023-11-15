//if:
//  'if' expression expression ('else' expression)?
use crate::lexer;
use super::{Expression, Parse, ParseError};

#[derive(Debug, Clone, PartialEq)]
struct IfExpr {
    condition: Expression,
    then_expression: Expression,
    else_expression: Option<Expression>,
}

impl IfExpr {
    fn new(condition: Expression, then_expression: Expression, else_expression: Option<Expression>) -> Self {
        IfExpr {
            condition,
            then_expression,
            else_expression,
        }
    }
}

impl Parse for IfExpr {
    fn parse(tokens: &mut Vec<lexer::Token>) -> Result<Self, ParseError>
        where
            Self: Sized {
        match tokens.last() {
            Some(token) => {
                if token.kind != lexer::TokenKind::Symbol || token.value != "if" {
                    return Err(ParseError::new("Expected 'if'", token.line, token.column));
                }
                tokens.pop();
            }
            None => {
                return Err(ParseError::new("Expected 'if'", 0, 0));
            }
        }

        let condition = Expression::parse(tokens)?;
        let then_expression = Expression::parse(tokens)?;
        let else_expression = match tokens.last() {
            Some(token) => {
                if token.kind == lexer::TokenKind::Symbol && token.value == "else" {
                    tokens.pop();
                    Some(Expression::parse(tokens)?)
                } else {
                    None
                }
            }
            None => None,
        };
        Ok(IfExpr::new(condition, then_expression, else_expression))
    }
}

#[cfg(test)]
mod test_if_expr {
    use super::*;

    #[test]
    fn returns_error_when_no_if() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "else".to_string(), 0, 0),
        ];
        let result = IfExpr::parse(&mut tokens);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn returns_error_when_no_condition() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "if".to_string(), 0, 0),
        ];
        let result = IfExpr::parse(&mut tokens);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn returns_error_when_no_then_expression() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "if".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 0, 0),
        ].into_iter().rev().collect();
        let result = IfExpr::parse(&mut tokens);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn parses_without_else() {
        let mut tokens = vec![
            lexer::Token::new(lexer::TokenKind::Symbol, "if".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Number, "1".to_string(), 0, 0),
            lexer::Token::new(lexer::TokenKind::Number, "2".to_string(), 0, 0),
        ].into_iter().rev().collect();
        let result = IfExpr::parse(&mut tokens);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), IfExpr::new(
            Expression::Number(1),
            Expression::Number(2),
            None,
        ));
    }
}