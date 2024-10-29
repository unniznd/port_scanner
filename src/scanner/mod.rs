use std::sync::mpsc::Sender;
use std::net:: IpAddr;
use std::io::{self, Write};

use tokio::net::TcpStream;

pub async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {
    // Attempts Connects to the address and the given port.
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {
        
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }
        Err(_) => {}
    }
}
