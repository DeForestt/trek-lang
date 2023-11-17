mod lexer;
mod parse;

fn main() {
    let tokens = match lexer::tokenize("let add = (a b) => print(\'Hello World\')".to_string()) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let module = parse::parse_module(&mut tokens.clone());
    match module {
        Ok(module) => {
            println!("{:?}", module);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
