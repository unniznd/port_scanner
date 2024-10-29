use std::{net::IpAddr, str::FromStr};

pub struct Command {
    pub addr: IpAddr,
    pub start_port: u16,
    pub end_port: u16
}

pub enum ParseResult {
    Ok(Command),
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

            let start_port = match args.next() {
                Some(s_port) => match s_port.parse::<u16>() {
                    Ok(e) => e,
                    Err(_) =>  return ParseResult:: Err("Error parsing start port")
                },
                None => 0,
            };
            let end_port = match args.next() {
                Some(s_port) => match s_port.parse::<u16>() {
                    Ok(e) => e,
                    Err(_) =>  return ParseResult:: Err("Error parsing end port")
                },
                None => 65535,
            };
    
        ParseResult::Ok(
            Command{addr, start_port, end_port}
        )
    
        
    }
}


