use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    //connect
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    // Trimite date
    //stream.write(b"Hello, server!").unwrap();

    //getting the answear
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();

    println!(
        "Server răspuns: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    // Close automat când stream iese din scope
}
