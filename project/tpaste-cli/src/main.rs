use std::io::{Read, Write, stdin};
use std::net::TcpStream;

fn main() {
    //connect
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    // Read welcome message from server
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(n) => {
            let message = String::from_utf8_lossy(&buffer[..n]);
            println!("{}", message);
        }
        Err(e) => eprintln!("Error reading from server: {}", e),
    }

    // Send username to server
    let mut username = String::new();
    println!("Enter username: ");
    match stdin().read_line(&mut username) {
        Ok(_) => match stream.write_all(username.as_bytes()) {
            Ok(_) => println!("Username sent"),
            Err(e) => eprintln!("Error sending username: {}", e),
        },
        Err(e) => eprintln!("Error reading username: {}", e),
    }

    // Send password to server
    let mut password = String::new();
    println!("Enter password: ");
    match stdin().read_line(&mut password) {
        Ok(_) => match stream.write_all(password.as_bytes()) {
            Ok(_) => println!("Password sent"),
            Err(e) => eprintln!("Error sending password: {}", e),
        },
        Err(e) => eprintln!("Error reading password: {}", e),
    }
}
