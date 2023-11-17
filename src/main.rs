mod lexer;
mod parse;
fn main() {
    let tokens = match lexer::tokenize("let add = (a b) => a".to_string()) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let expression = match parse::Expression::parse(&mut tokens.clone()) {
        Ok(expression) => expression,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    println!("{:?}", expression);
}
