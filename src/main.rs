use io::Read;
use io::Write;
use std::io;
use std::process;
use std::str;
mod prints;
mod recieve;
mod send;

fn main() {
    let mut input = String::new(); // read exactly one byte
    prints::prompt("Is this device sending or receiving? (s/r) ");
    std::io::stdin().read_line(&mut input);
    let result = input.chars().next().unwrap();
    if result == 's' {
        send::send();
    } else if result == 'r' {
        recieve::recieve();
    } else {
        prints::error(&*format!("Invalid input: {}", input));
        process::exit(1);
    }
    
}
