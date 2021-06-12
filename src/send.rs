use crate::prints;
use io::prelude::*;
use io::BufRead;
use std::fs::File;
use std::io;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;

pub fn send() {
    let stdin = io::stdin();
    prints::info("Starting sending process");
    let ip = local_ipaddress::get().unwrap();
    prints::prompt("Enter path to file:");
    let path = stdin.lock().lines().next().unwrap().unwrap();
    prints::info(&*format!("Address: {}", ip));
    let listener = TcpListener::bind(format!("{}:7878", ip)).unwrap();
    prints::info("Waiting for reciever");

    for stream in listener.incoming() {
        prints::info("Reciever detected!");
        let stream = stream.unwrap();

        handle_connection(stream, &path);
        break;
    }
}

fn handle_connection(mut stream: TcpStream, path: &str) {
    prints::info("Sending file...");
    if let Ok(lines) = read_lines(&path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(linestr) = line {
                stream.write(linestr.as_bytes()).unwrap();
                stream.write("\n\r".as_bytes()).unwrap();
            }
        }
    } else {
        prints::error("WHAT HAPPENED?");
    }
    stream.flush().unwrap();
    prints::info("Sending complete!");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
