mod db_functions;
mod db_model;
mod helper_funcions;
use crate::db_functions::Database;
use crate::helper_funcions::validate_password;

use crate::helper_funcions::{generate_paste_code, hash_password, validate_username};

//use crate::sign_up::sign_up;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;

fn read_line(stream: &mut TcpStream) -> String {
    let mut line = String::new();
    let mut buf = [0; 1];

    // reading byte by byte until we find endl
    while stream.read_exact(&mut buf).is_ok() {
        if buf[0] == b'\n' {
            break;
        }
        line.push(buf[0] as char);
    }
    line.trim().to_string()
}

fn handle_client(mut stream: TcpStream, db: Database) {
    let mut authenticated_user_id: Option<i64> = None; // Remember the user ID for subsequent operations
    let mut connected: bool = false;

    // Make tpaste command unavailable until the user is logged in
    while !connected {
        let command = read_line(&mut stream);
        if command.is_empty() {
            break;
        } //closing connection

        // Process the command
        match command.as_str() {
            "sign_up" => {
                // Request username and password
                let username = read_line(&mut stream);
                let password = read_line(&mut stream);

                // Validate username
                if let Err(e) = validate_username(&username) {
                    stream
                        .write_all(format!("ERR: Username invalid: {}\n", e).as_bytes())
                        .unwrap();

                    continue;
                }
                // Check if the username already exists
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

                match validate_password(&password) {
                    // Insert the new user in the database
                    Ok(message) => match db.sign_up(username, password) {
                        Ok(id) => {
                            authenticated_user_id = Some(id);
                            connected = true;
                            let _ = stream.write(message.as_bytes());
                        }
                        Err(e) => {
                            stream
                                .write_all(format!("ERR: Signup failed: {}\n", e).as_bytes())
                                .unwrap();
                        }
                    },
                    Err(e) => {
                        let response = format!("Err: {}\n, Sign_up failed.", e);
                        stream.write_all(response.as_bytes()).unwrap();
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

                        // Generate a new token
                        let new_token = helper_funcions::generate_auth_token();
                        db.create_token(id, &new_token).unwrap();

                        // Send message with the format the client expects: "TOKEN:..."
                        let success_msg = format!("OK: Login successful! TOKEN:{}\n", new_token);
                        stream.write_all(success_msg.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        stream
                            .write_all(format!("ERR: Login failed: {}\n", e).as_bytes())
                            .unwrap();
                    }
                }
            }
            "token" => {
                let token_raw = read_line(&mut stream);
                let token = token_raw.trim();

                match db.validate_token(token) {
                    Ok(Some(id)) => {
                        authenticated_user_id = Some(id);
                        connected = true; // This will exit the while loop and enter the next loop
                        let _ = stream.write_all(b"OK: Welcome back via token!\n");
                    }
                    _ => {
                        let _ = stream.write_all(b"ERR: Token invalid or expired.\n");
                    }
                }
            }
            "exit" => {
                stream.write_all(b"Goodbye!").unwrap();
                break; // Close the connection
            }
            other => {
                eprintln!("User tried to use {} command", other);
                stream
                    .write_all(b"Unknown command. Try 'sign_up' or 'login'.\n")
                    .unwrap();
            }
        }
    }
    let message = "you are logged in,now you have accest to tpaste server";
    stream.write_all(message.as_bytes()).unwrap();
    loop {
        let command = read_line(&mut stream);
        let executable_command = command.trim();
        if executable_command.is_empty() {
            let _ = stream.write_all(b"you enterd an empty command, try to use help");
        } else {
            let _ = executable_command.trim();
            if executable_command.ends_with("| tpaste") {
                let cmd_to_run = executable_command
                    .replace("| tpaste", "")
                    .trim()
                    .to_string();

                match Command::new("sh").arg("-c").arg(&cmd_to_run).output() {
                    Ok(output) => {
                        //combine stdin and stderr to save everything the command has shown
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let content = format!("{}{}", stdout, stderr);

                        if content.trim().is_empty() {
                            let _ = stream.write_all(
                                b"Command was executed but it didn't produce any output",
                            );
                        } else {
                            let code = generate_paste_code();

                            match db.create_paste(&authenticated_user_id.unwrap(), &code, &content)
                            {
                                Ok(id) => {
                                    let message = format!(
                                        "Message saved with code: {},your user id: {}",
                                        code, id
                                    );
                                    stream.write_all(message.as_bytes()).unwrap();
                                }
                                Err(e) => {
                                    let err_msg = format! {"Err:Something went wrong\n {}",e};
                                    stream.write_all(err_msg.as_bytes()).unwrap();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let err_msg = format!("ERR: Failed to execute command: {}\n", e);
                        stream.write_all(err_msg.as_bytes()).unwrap();
                    }
                }
            }
        }
        if executable_command.contains("link:") {
            let code = executable_command.replace("link:", "").trim().to_string();
            //looking for paste
            match db.get_paste_by_code(&code) {
                Ok(paste) => {
                    //search the user by user id
                    match db.get_user_id(paste.user_id) {
                        Ok(user) => {
                            //format the message
                            let response = format!(
                                "Author:{} \n Date:{}\nContent:{}",
                                user.username,
                                paste.created_at.format("%Y-%m-%d %H:%M:%S"),
                                paste.content
                            );
                            stream.write_all(response.as_bytes()).unwrap()
                        }
                        Err(e) => {
                            let response = format!(
                                "ERR:{}:The author of this paste doesn't exist anymore,\n",
                                e
                            );
                            stream.write_all(response.as_bytes()).unwrap();
                        }
                    }
                }
                Err(e) => {
                    let respone = format!("Err: {},Your provided code doesn't exist\n", e);
                    stream.write_all(respone.as_bytes()).unwrap();
                }
            }
        }
        if executable_command == "my_pastes" {
            match db.get_user_pastes(authenticated_user_id.unwrap()) {
                Ok(history) => {
                    stream.write_all(history.as_bytes()).unwrap();
                }
                Err(e) => {
                    stream
                        .write_all(format!("ERR: Nu am putut prelua istoricul: {}\n", e).as_bytes())
                        .unwrap();
                }
            }
        }
        if executable_command.contains("exit") {
            stream.write_all(b"Goodbye!").unwrap();
            break; // Close the connection
        }
        if executable_command.contains("help") {
            stream
                .write_all(b"Availble commands are link:code,my_pastes and | tpaste")
                .unwrap();
        }
    }
}
fn main() {
    let db = Database::new("tpaste.db").expect("Error: cannot create the database");

    // Create socket, bind, and listen
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server started");
    // Accept incoming connections
    for stream in listener.incoming() {
        let db_client = db.clone();
        match stream {
            Ok(stream) => {
                // Concurrent server using threads
                thread::spawn(move || {
                    handle_client(stream, db_client);
                });
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
