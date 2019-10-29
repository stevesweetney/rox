use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;

mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod statement;
mod token;

#[cfg(test)]
mod test;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 2 {
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt();
    }

    Ok(())
}

fn run_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    run(buffer.clone(), &mut Interpreter::default());

    Ok(())
}

fn run_prompt() {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::default();
    loop {
        match rl.readline("> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                run(line, &mut interpreter);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

fn run(source: String, interpreter: &mut Interpreter) {
    let mut s = Scanner::new(source);
    let tokens = s.scan_tokens().to_vec();
    let mut parser = Parser::new(tokens);

    if let Err(e) = parser.parse().and_then(|stms| interpreter.interpret(&stms)) {
        eprintln!("{}", e)
    }
}
