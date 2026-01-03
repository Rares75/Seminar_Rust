use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
fn handle_client(mut stream: TcpStream) {
    let message = "connected client, welocome to my first rust server";
    stream.write_all(message.as_bytes());
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
