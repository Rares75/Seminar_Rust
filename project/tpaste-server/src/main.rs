mod db_functions;
mod db_model;
mod helper_funcions;
mod login;
//mod sign_up;
use crate::db_functions::Database;
use crate::db_model::{Paste, Token, User};
use crate::helper_funcions::{
    generate_auth_token, generate_paste_code, hash_password, validate_password, validate_username,
};
use crate::login::login;
//use crate::sign_up::sign_up;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn read_from_stream(stream: &mut TcpStream) -> String {
    let mut buf = [0; 512];
    let n = stream.read(&mut buf).unwrap_or(0);
    String::from_utf8_lossy(&buf[..n]).trim().to_string()
}

fn handle_client(mut stream: TcpStream, db: Database) {
    let mut buffer = [0; 512];
    let mut authenticated_user_id: Option<i64> = None; //remembering the user id for the rest of the functions
    let mut connected: bool = false;
    let mut command: String = String::new();

    stream.write_all(b"Welcome to the Tpaste web server, before we start plese login using login command\n If you don't have an account please use sign_up command\n If you want to quit just use exit command\n");

    //making the tpaste command unavailable until the user is logged in
    while !connected {
        buffer.fill(0);
        let n = match stream.read(&mut buffer) {
            Ok(0) => break, //client closed the connection
            Ok(n) => n,
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        };

        command = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

        //processing the command
        match command.as_str() {
            "sign_up" => {
                //asking for the username and password
                stream.write_all(b"Enter username: ").unwrap();
                let mut username = read_from_stream(&mut stream);
                loop {
                    if let Err(e) = validate_username(&username) {
                        stream
                            .write_all(format!("Invalid username: {}. Try again.\n", e).as_bytes())
                            .unwrap();
                        username = read_from_stream(&mut stream);
                    } else {
                        break; //good username
                    }
                }

                stream.write_all(b"Enter password: ").unwrap();
                let mut password = read_from_stream(&mut stream);
                loop {
                    if let Err(e) = validate_password(&password) {
                        stream
                            .write_all(format!("Invalid password: {}. Try again.\n", e).as_bytes())
                            .unwrap();
                        password = read_from_stream(&mut stream);
                    } else {
                        break; //good password
                    }
                }
                match db.sign_up(username, password) {
                    Ok(id) => {
                        buffer.fill(0);
                        stream.write_all(b"would you like to generate a token? y/n");
                        buffer.fill(0);
                        let n = stream.read(&mut buffer).unwrap();
                        let answear = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                        println! {"the answear is: {}",answear};
                        authenticated_user_id = Some(id);
                        connected = true;
                    }
                    Err(e) => {
                        stream.write_all(e.as_bytes()).unwrap();
                        stream.write_all(b"\n").unwrap();
                    }
                }
            }
            "login" => {
                stream.write(b"login function will come soon");
            }
            "exit" => {
                stream.write_all(b"Goodbye!").unwrap();
                break; //closing the connection
            }
            other => {
                eprintln! {"the user tried to use {} command", other};
                stream
                    .write_all(b"Unknown command. Try 'sign_up' or 'login'.\n")
                    .unwrap();
            }
        }
    }
    loop {
        stream.write_all(b"you are now connected, rest of the commands will come soon...");
        break;
    }
}
fn main() {
    let db = Database::new("tpaste.db").expect("Error:can not create the DB");

    //socket+bind+listen()
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println! {"server started"};
    // accept
    for stream in listener.incoming() {
        let db_client = db.clone();
        match stream {
            Ok(stream) => {
                // concurrent server with Threads
                thread::spawn(move || {
                    handle_client(stream, db_client);
                });
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
