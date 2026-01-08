use std::io::{self, Read, Write, stdin, stdout};
use std::net::TcpStream;
use std::process::Command;
/*fn main() {
    //connect
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("connection refused");
    println!("Connected to tpaste server. Type 'help' for commands.");
    // Read welcome message from server
    //let mut buffer = [0; 512];
    //let mut message: String = String::new();

    loop {
        print!("tpaste> ");
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
}*/
fn read_server_response(stream: &mut TcpStream) {
    let mut buffer = [0; 4096];
    match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            println!("\n{}", String::from_utf8_lossy(&buffer[..n]));
        }
        _ => println!("Error reading from server."),
    }
}
fn handle_auth(stream: &mut TcpStream, cmd_type: &str) {
    let mut username = String::new();
    let mut password = String::new();

    print!("Enter username: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut username).unwrap();

    print!("Enter password: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut password).unwrap();

    //sending the entire package to the server, the format is command\nusername\npassword
    let payload = format!("{}\n{}\n{}\n", cmd_type, username.trim(), password.trim());
    stream.write_all(payload.as_bytes()).unwrap();

    //getting the answear from the server
    read_server_response(stream);
}
fn handle_post(stream: &mut TcpStream, code: &str) {
    // sending get command followed by the code
    let payload = format!("get\n{}\n", code);
    stream.write_all(payload.as_bytes()).unwrap();

    read_server_response(stream);
}
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Connection refused");
    println!("Connected to tpaste server. Type 'help' for commands.");
    //check if stdin get the data from a pipe(cat file | tpaste)
    /*if !stdin().is_terminal() {
        let mut buffer = String::new();
        if stdin().read_to_string(&mut buffer).is_ok() && !buffer.is_empty() {
            handle_post_locally(&mut stream, buffer);
            return; //after we use tpaste, we close the program to emulate a linux cmd
        }
    }*/

    loop {
        print!("tpaste> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim();

        if command.is_empty() {
            println!(
                "You introduce an empty command, use help function to see the availble commands!"
            );
            continue;
        }
        if command == "exit" || command == "quit" {
            break;
        }

        match command {
            "help" => {
                println!("Available commands: login, signup, tpaste, get, exit")
            }
            "login" => handle_auth(&mut stream, "login"),
            "sign_up" => handle_auth(&mut stream, "sign_up"),

            "link" => {
                let code = command.replace("link:", "");

                handle_post(&mut stream, &code);
            }
            _ => {
                //any other command will be executed normally
                let status = Command::new("sh").arg("-c").arg(command).status();

                if let Err(e) = status {
                    println!("Eroare la executarea comenzii: {}", e);
                }
            }
        }
    }
    println!("Goodbye!");
}
