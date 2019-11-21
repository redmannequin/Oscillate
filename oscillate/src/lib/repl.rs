use std::io::{self, Write};
use parser::Lexer;
use parser::Parser;

use crate::error::Result;

pub fn start() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    loop {
        input.clear();
        print!(">> ");
        stdout.flush().expect("Error faild to flush");
        stdin.read_line(&mut input).expect("Error reading from STDIN");
        
        let lexer = Lexer::new(input.as_str());
        let mut parser = Parser::new(lexer);
        let program = parser.parse()?;

        // let errors = parser.clear_errors();
        // if errors.len() > 0 {
        //     println!("parser errors:");
        //     for error in errors {
        //         println!("  {:?}", error);
        //     }
        //     continue;
        // }

        println!("{:?}", program);
    }
}