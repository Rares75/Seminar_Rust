use std::io::{self, Read, Write, stdin, stdout};
use std::net::TcpStream;

fn main() {
    //connect
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("connection refused");

    // Read welcome message from server
    let mut buffer = [0; 512];
    let mut message: String = String::new();

    loop {
        //read the message from server
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("The server closed the connection.");
                break;
            }
            Ok(n) => {
                message = String::from_utf8_lossy(&buffer[..n]).to_string();
                io::stdout().flush().unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from server: {}", e);
                break;
            }
        }
        //printing the message
        println! {"{}",message};
        message.clear();

        //reading the new command for the server
        stdin()
            .read_line(&mut message)
            .expect("Error at reading your message");

        //sending the command to the server
        if let Err(e) = stream.write_all(message.as_bytes()) {
            eprintln!("Eroare la trimitere: {}", e);
            break;
        }
    }
}
