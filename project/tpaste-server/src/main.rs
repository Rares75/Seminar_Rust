mod login;
use crate::login::login;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut username: String = String::new();
    let mut password: String = String::new();
    let message = "welcome, please login so you can use tpaste command\n";
    stream.write_all(message.as_bytes());
    let mut buffer = [0; 256]; // buffer of 256 bytes
    match stream.read(&mut buffer) {
        Ok(n) => {
            username = String::from_utf8_lossy(&buffer[..n]).to_string();
            // now you have the username as a String
        }
        Err(e) => eprintln!("Error reading: {}", e),
    }
    println! {"i recieved the username: {}",{username.clone()}};
    buffer.fill(0);
    stream.write_all("password: ".as_bytes());
    match stream.read(&mut buffer) {
        Ok(n) => {
            password = String::from_utf8_lossy(&buffer[..n]).to_string();
        }
        Err(e) => eprintln!("Error reading: {}", e),
    }
    println! {"I recieved the password: {}",{password.clone()}};
    login(username, password);
}
fn main() {
    //socket+bind+listen()
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println! {"server started"};
    // accept
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // concurrent server with Threads
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
