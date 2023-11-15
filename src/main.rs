mod lexer;
mod parse;
fn main() {
    let tokens = match lexer::tokenize("let add = a b => a + b".to_string()) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    println!("{:?}", tokens);
}
