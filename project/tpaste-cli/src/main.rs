use std::io::{self, IsTerminal, Read, Write, stdin, stdout};
use std::net::TcpStream;

// Funcție pentru a citi și afișa TOT ce trimite serverul (inclusiv output-uri lungi de la ls/cat)
fn read_server_response(stream: &mut TcpStream) {
    let mut buffer = [0; 8192]; // Buffer mai mare pentru output-uri de comenzi
    match stream.read(&mut buffer) {
        Ok(n) => {
            let response = String::from_utf8_lossy(&buffer[..n]);
            print!("{}", response); // Folosim print! nu println! pentru că serverul trimite deja \n
            io::stdout().flush().unwrap();
        }
        Ok(0) => println!("\nServer disconnected."),
        Err(e) => println!("\nError reading from server: {}", e),
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

    // Trimitem: comanda\nusername\nparola\n
    let payload = format!("{}\n{}\n{}\n", cmd_type, username.trim(), password.trim());
    stream.write_all(payload.as_bytes()).unwrap();

    read_server_response(stream);
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Connection refused");

    // 1. GESTIONARE PIPE (Ex: cat file | tpaste)
    if !stdin().is_terminal() {
        let mut buffer = String::new();
        if stdin().read_to_string(&mut buffer).is_ok() {
            // Dacă e pipe, trimitem direct conținutul ca o comandă de tip tpaste
            // (Presupunând că serverul știe să identifice acest flux)
            stream
                .write_all(format!("{} | tpaste\n", buffer.trim()).as_bytes())
                .unwrap();
            read_server_response(&mut stream);
        }
        return;
    }

    println!("TPaste Remote Shell Connected.");

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
            // Cazurile speciale unde clientul ajută la introducerea datelor
            "login" | "sign_up" => handle_auth(&mut stream, command),

            // Orice altceva (ls, link:abc, pwd, echo "hi" | tpaste)
            // Trimitem exact ce a scris utilizatorul la server
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
