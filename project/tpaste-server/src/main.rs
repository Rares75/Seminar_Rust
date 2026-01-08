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

fn read_line(stream: &mut TcpStream) -> String {
    let mut line = String::new();
    let mut buf = [0; 1];

    // reading byte by byte until we find endl
    loop {
        match stream.read_exact(&mut buf) {
            Ok(_) => {
                if buf[0] == b'\n' {
                    break;
                }
                line.push(buf[0] as char);
            }
            Err(_) => break,
        }
    }
    line.trim().to_string()
}

fn handle_client(mut stream: TcpStream, db: Database) {
    let mut authenticated_user_id: Option<i64> = None; //remembering the user id for the rest of the functions
    let mut connected: bool = false;
    let mut command: String = String::new();

    //making the tpaste command unavailable until the user is logged in
    while !connected {
        command = read_line(&mut stream);
        if command.is_empty() {
            break;
        } //closing connection

        //processing the command
        match command.as_str() {
            "sign_up" => {
                //asking for the username and password
                let mut username = read_line(&mut stream);
                let mut password = read_line(&mut stream);

                //validate username
                if let Err(e) = validate_username(&username) {
                    stream
                        .write_all(format!("ERR: Username invalid: {}\n", e).as_bytes())
                        .unwrap();

                    continue;
                }
                // check if the username already exists
                let exists = match db.username_exists(&username) {
                    Ok(true) => true,
                    Ok(false) => false,
                    Err(e) => {
                        stream
                            .write_all(format!("ERR: DB error: {}\n", e).as_bytes())
                            .unwrap();
                        continue;
                    }
                };

                if exists {
                    stream.write_all(b"ERR: Username already exists\n").unwrap();
                    continue;
                }

                // insert the new user in the DB
                match db.sign_up(username, password) {
                    Ok(id) => {
                        authenticated_user_id = Some(id);
                        connected = true;
                        stream
                            .write_all(b"OK: Account created and logged in.\n")
                            .unwrap();
                    }
                    Err(e) => {
                        stream
                            .write_all(format!("ERR: Signup failed: {}\n", e).as_bytes())
                            .unwrap();
                    }
                }
            }
            "login" => {
                let username = read_line(&mut stream);
                let password = read_line(&mut stream);

                match db.login(username, password) {
                    Ok(id) => {
                        authenticated_user_id = Some(id);
                        connected = true;
                        stream.write_all(b"OK: Login successful!\n").unwrap();
                    }
                    Err(e) => {
                        stream
                            .write_all(format!("ERR: Login failed: {}\n", e).as_bytes())
                            .unwrap();
                    }
                }
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
