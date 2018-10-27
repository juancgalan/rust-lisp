extern crate rustyline;

mod parser;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    println!("Risp Version 0.0.1");
    println!("Press Ctrl+c to Exit");
    let mut rl = Editor::<()>::new();
    if rl.load_history("risp_history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let line = rl.readline("risp>> ");
        match line {
            Ok(e) => {
                rl.add_history_entry(e.as_ref());
                println!("No you're a {}", e);
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("risp_history.txt").unwrap();
}
