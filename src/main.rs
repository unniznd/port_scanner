mod parser;
mod scanner;

use std::env;
use std::sync::mpsc::channel;
use std::thread;

use parser::Command;
use parser::ParseResult;
use scanner::scan;

fn main() {
    match Command::from_str(env::args()) {
        ParseResult::Ok(command) => {
            let num_threads = command.threads;
            let addr = command.addr;
            let (tx, rx) = channel();
            for i in 0..num_threads {
                let tx = tx.clone();

                thread::spawn(move || {
                    scan(tx, i, addr, num_threads);
                });
            }

            let mut out = vec![];
            drop(tx);
            for p in rx {
                out.push(p);
            }

            println!("");
            out.sort();
            for v in out {
                println!("{} is open", v);
            }
        }
        ParseResult::Help(message) => {
            println!("{}", message);
        }
        ParseResult::Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
