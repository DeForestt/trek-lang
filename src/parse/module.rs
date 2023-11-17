// module: expression+ EOF

use super::{Expression, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub expressions: Vec<Expression>
}

impl Module {
    pub fn new(expressions: Vec<Expression>) -> Self {
        Module { expressions }
    }
}

impl Parse for Module {
    fn parse( tokens: &mut Vec<crate::lexer::Token>) -> Result<Self, super::ParseError>
        where
            Self: Sized {
        let mut expressions = Vec::new();
        loop {
            match tokens.last() {
                Some(_token ) => {
                    expressions.push(Expression::parse(tokens)?);
                }
                None => {
                    break;
                }
            }
        }
        if expressions.len() == 0 {
            return Err(super::ParseError::new("Expected expression", 0, 0));
        }
        Ok(Module::new(expressions))
    }
}
