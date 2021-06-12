use crate::prints;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn recieve() {
    let stdin = io::stdin();
    prints::info("Starting recieving process");
    prints::prompt("Enter adress given on sending machine:");
    let ip = stdin.lock().lines().next().unwrap().unwrap();
    let mut stream = TcpStream::connect(format!("{}:7878", ip)).unwrap();
    let mut file = String::new();

    prints::info("Recieveing file...");
    stream.write(&[1]);
    stream.read_to_string(&mut file);
    println!("{}", file);
}
