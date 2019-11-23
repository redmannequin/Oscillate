use std::io;
use std::io::Write;

use parser::Lexer;
use parser::Parser;
use parser::eval;
use parser::Env;
use parser::traits::Container;

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    let env = Container::new(Env::new());

    loop {
        input.clear();
        print!(">> ");
        stdout.flush().expect("Error faild to flush");
        stdin.read_line(&mut input).expect("Error reading from STDIN");
        
        let lexer = Lexer::new(input.as_str());
        let mut parser = Parser::new(lexer);
        let program = parser.parse().unwrap();

        match eval(&program, env.clone()) {
            Ok(obj) => println!("{:?}", obj),
            Err(err) => println!("Error: {:?}", err)
        }
        
    }
}