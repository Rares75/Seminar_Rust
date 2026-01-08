use std::io::{self, IsTerminal, Read, Write, stdin, stdout};
use std::net::TcpStream;

fn handle_auth(stream: &mut TcpStream, cmd_type: &str) {
    let mut username = String::new();
    let mut password = String::new();

    print!("Enter username: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut username).unwrap();

    print!("Enter password: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut password).unwrap();

    // Send: command\nusername\npassword\n
    let payload = format!("{}\n{}\n{}\n", cmd_type, username.trim(), password.trim());
    stream.write_all(payload.as_bytes()).unwrap();

    read_server_response(stream);
}

fn read_server_response(stream: &mut TcpStream) {
    let mut buffer = [0; 4096];
    if let Ok(n) = stream.read(&mut buffer) {
        if n > 0 {
            let response = String::from_utf8_lossy(&buffer[..n]);
            println!("{}", response);

            // If the server sent us a token, save it
            if response.contains("TOKEN:") {
                if let Some(token) = response.split("TOKEN:").nth(1) {
                    let clean_token = token.trim();
                    std::fs::write(".tpaste_token", clean_token).expect("Failed to save token");
                    println!("Session saved locally.");
                }
            }
        }
    }
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Connection refused");
    println!("Connected to tpaste server. Type 'help' for commands.");
    // 1. HANDLE PIPE INPUT (Ex: cat file | tpaste)
    if !stdin().is_terminal() {
        let mut buffer = String::new();
        if stdin().read_to_string(&mut buffer).is_ok() {
            // If input is piped, send it directly as a tpaste command
            // (Assuming the server knows how to identify this flow)
            stream
                .write_all(format!("{} | tpaste\n", buffer.trim()).as_bytes())
                .unwrap();
            read_server_response(&mut stream);
        }
        return;
    }
    if std::path::Path::new(".tpaste_token").exists() {
        if let Ok(token) = std::fs::read_to_string(".tpaste_token") {
            let payload = format!("token\n{}\n", token.trim());
            stream.write_all(payload.as_bytes()).unwrap();

            // Read the response to check if we succeeded
            let mut buf = [0; 512];
            if let Ok(n) = stream.read(&mut buf) {
                let res = String::from_utf8_lossy(&buf[..n]);
                if res.contains("OK:") {
                    println!("Automatically logged in via token.");
                } else {
                    println!("Token expired. Please log in manually.");
                }
            }
        }
    }
    loop {
        print!("tpaste> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim();

        if command.is_empty() {
            continue;
        }
        if command == "exit" || command == "quit" {
            break;
        }

        match command {
            // Special cases where the client helps with data input
            "login" | "sign_up" => handle_auth(&mut stream, command),

            // Anything else (ls, link:abc, pwd, echo "hi" | tpaste)
            // Send exactly what the user typed to the server
            _ => {
                stream
                    .write_all(format!("{}\n", command).as_bytes())
                    .unwrap();
                read_server_response(&mut stream);
            }
        }
    }
    println!("Goodbye!");
}
