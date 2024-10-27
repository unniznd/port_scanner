mod parser;

use std::env;

use parser::Command;
use parser::ParseResult;

fn main() {
    match Command::from_str(env::args()) {
        ParseResult::Ok(command) => {
            println!("{} {:?}", command.addr,command.threads);
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
