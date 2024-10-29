mod parser;
mod scanner;

use std::env;
use std::sync::mpsc::channel;

use tokio::task;

use parser::Command;
use parser::ParseResult;
use scanner::scan;


#[tokio::main]
async fn main() {
    match Command::from_str(env::args()) {
        ParseResult::Ok(command) => {
            let addr = command.addr;
            let (tx, rx) = channel();
            for i in command.start_port..command.end_port {
                let tx = tx.clone();

                task::spawn(async move { 
                    scan(tx, i, addr).await 
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
        ParseResult::Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
