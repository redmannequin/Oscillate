use std::io;
use std::io::Write;

use interpreter::Evaluator;

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let env = Evaluator::new();

    loop {
        let mut input = String::new();
        print!(">> ");
        stdout.flush().expect("Error faild to flush");
        stdin.read_line(&mut input).expect("Error reading from STDIN");
        
        match env.eval(input)  {
            Ok(obj) => println!("{:?}", obj),
            Err(err) => println!("Error: {:?}", err)
        }
    }
}