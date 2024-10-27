use std::{net::IpAddr, str::FromStr};
#[derive(Debug, PartialEq)]
pub struct Command {
    pub addr: IpAddr,
    pub threads: Option<u16>,
}

pub enum ParseResult {
    Ok(Command),
    Help(&'static str),
    Err(&'static str),
}


impl Command {
    pub fn from_str(mut args: impl Iterator<Item = String>) -> ParseResult {
        args.next();

        let addr = match args.next()
            .ok_or("No IP Address found")
            .and_then(|val| IpAddr::from_str(&val).map_err(|_| "Invalid IP Address")) {
                Ok(addr) => addr,
                Err(e) => return ParseResult::Err(e),
            };

        let threads = match args.next().as_deref() {
            Some("-t") => {
                match args.next()
                    .ok_or("Missing thread input")
                    .and_then(|val| val.parse::<u16>()
                    .map_err(|_| "Invalid thread input")) {
                        Ok(threads) => Some(threads),
                        Err(e) => return ParseResult::Err(e),
                     }
            }
            Some("-h") => return ParseResult:: Help(
                "Usage: command <IP_ADDRESS> [-t <THREAD_COUNT>] [-h for help]"
            ),
            Some(_) => return ParseResult:: Err("Invalid parameter"),
            None => None, 
        };
    
        ParseResult::Ok(
            Command{addr, threads}
        )
    
        
    }
}


