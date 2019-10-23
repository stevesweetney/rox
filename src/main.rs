use std::env;
use std::fs::File;
use std::io::{self, stdin, Read, Write};
use std::path::Path;
use std::process;

mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;

use parser::Parser;
use scanner::Scanner;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 2 {
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    run(buffer.clone());

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        run(buffer.clone());

        buffer.clear();
    }
}

fn run(source: String) {
    let mut s = Scanner::new(source);
    let tokens = s.scan_tokens().to_vec();
    let mut parser = Parser::new(tokens);

    if let Ok(expression) = parser.parse() {
        match interpreter::interpret(expression) {
            Ok(val) => println!("{}", val),
            Err(e) => eprintln!("{}", e),
        }
    }
}
