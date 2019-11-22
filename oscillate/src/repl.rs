use std::io::{self, Write};
use parser::Lexer;
use parser::Parser;

use std::rc::Rc;
use std::cell::RefCell;

use eval::run;
use eval::Environment;

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    let env = Rc::new(RefCell::new(Environment::new()));

    loop {
        input.clear();
        print!(">> ");
        stdout.flush().expect("Error faild to flush");
        stdin.read_line(&mut input).expect("Error reading from STDIN");
        
        let lexer = Lexer::new(input.as_str());
        let mut parser = Parser::new(lexer);
        let program = parser.parse().unwrap();

        match run(&program, Rc::clone(&env)) {
            Ok(obj) => println!("{:?}", obj),
            Err(err) => println!("Error: {:?}", err)
        }
        
    }
}