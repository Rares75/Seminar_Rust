use std::collections::HashMap;
use std::fs;
use std::io::stdin;

use rusqlite::Connection;

trait EmulateCommand {
    fn get_name(&self) -> String;
    fn exec(&mut self, args: &[String]) -> Result<(), String>;
}
trait TerminalCommands {
    fn new() -> Self;
    fn register(&mut self, commands: Box<dyn EmulateCommand>);
    fn run(&mut self);
}
struct Terminal {
    commands: HashMap<String, Box<dyn EmulateCommand>>,
}
struct PingCommand {}
struct CountCommand {}
struct TimesCommand {
    count: u32,
}
struct HelloCommand {}
impl TerminalCommands for Terminal {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
    fn register(&mut self, commands: Box<dyn EmulateCommand>) {
        self.commands
            .insert(commands.get_name().to_string(), commands);
    }
    fn run(&mut self) {
        let termianl_commands = match fs::read_to_string("test.txt") {
            Ok(terminal_commands) => terminal_commands,
            Err(e) => {
                eprintln!("Eroare {} la citirea din fisier", e);
                return;
            }
        };
        for line in termianl_commands.lines() {
            let command = line.trim();
            if command.is_empty() {
                continue;
            }

            let parts: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
            let command_name = &parts[0];
            let args = &parts[1..];
            if command_name == "stop" {
                println!("terminalul isi termina executia");
                break;
            }
            match self.commands.get_mut(command_name) {
                Some(command) => {
                    if let Err(e) = command.exec(args) {
                        eprintln!("eroare {} la comanda {}", e, command_name);
                    }
                }
                None => {
                    let new_command = command_name.to_lowercase();
                    if self.commands.contains_key(&new_command) {
                        eprintln!(
                            "command unavailble,maybe u wanted to use {} command",
                            new_command
                        );
                    } else {
                        eprintln!(
                            "Command not found!Available commands are: ping,count(args), times"
                        );
                    }
                }
            }
        }
    }
}
impl EmulateCommand for PingCommand {
    fn get_name(&self) -> String {
        "ping".to_string()
    }
    fn exec(&mut self, _args: &[String]) -> Result<(), String> {
        println!("pong!");
        Ok(())
    }
}
impl EmulateCommand for CountCommand {
    fn get_name(&self) -> String {
        "count".to_string()
    }
    fn exec(&mut self, args: &[String]) -> Result<(), String> {
        let number = args.len();

        println!("{}", number);
        Ok(())
    }
}
impl EmulateCommand for TimesCommand {
    fn get_name(&self) -> String {
        "times".to_string()
    }
    fn exec(&mut self, _args: &[String]) -> Result<(), String> {
        self.count += 1;
        println!("{}", self.count);
        Ok(())
    }
}
impl EmulateCommand for HelloCommand {
    fn get_name(&self) -> String {
        "hello".to_string()
    }
    fn exec(&mut self, _args: &[String]) -> Result<(), String> {
        let mut name: String = String::new();
        stdin().read_line(&mut name).expect("input error");

        println!("hello {}", name);
        Ok(())
    }
}

//problem 2 struct and trait;
struct BKCommand {
    conn: rusqlite::Connection,
}
impl BKCommand {
    fn new() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open("bookmarks.db")?;
        let create = r"
     create table if not exists bookmarks (
    name text    not null,
    URL text not null
    )";
        conn.execute(create, ())?;
        Ok(Self { conn })
    }
}

impl EmulateCommand for BKCommand {
    fn get_name(&self) -> String {
        "bk".to_string()
    }

    fn exec(&mut self, args: &[String]) -> Result<(), String> {
        let command = args.first().map(|s| s.as_str());
        match command {
            Some("add") => {
                if let (Some(name), Some(url)) = (args.get(1), args.get(2)) {
                    self.conn
                        .execute(
                            "INSERT INTO bookmarks (name, url) VALUES (?1, ?2);",
                            (name, url),
                        )
                        .map_err(|e| e.to_string())?;
                    Ok(())
                } else {
                    Err("error at name or url".to_string())
                }
            }
            Some("search") => {
                if let Some(name) = args.get(1) {
                    let search = format!("%{}%", name);
                    let mut stmt = self
                        .conn
                        .prepare("SELECT name, url FROM bookmarks WHERE name LIKE ?1") //unclosed parameter error
                        .map_err(|e| e.to_string())?;
                    let bk_iter = stmt
                        .query_map([&search], |row| Ok((row.get(0)?, row.get(1)?)))
                        .map_err(|e| e.to_string())?;
                    for i in bk_iter {
                        let (name, url): (String, String) = i.map_err(|e| e.to_string())?;
                        println!("-> Nume: {}, URL: {}", name, url);
                    }
                    Ok(())
                } else {
                    Err("Utilizare: bk search <termen>".to_string())
                }
            }
            Some(unknown) => Err(format!("Sub-comanda necunoscută: '{}'", unknown)),
            None => Err(
                "Command not found, you can use bk add <name> <URL> or bk search <name>"
                    .to_string(),
            ),
        }
    }
}
fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(HelloCommand {}));

    match BKCommand::new() {
        Ok(mut bk) => {
            let _ = bk.exec(&[
                "add".to_string(),
                "Rust".to_string(),
                "https://rust-lang.org".to_string(),
            ]);
            let _ = bk.exec(&[
                "add".to_string(),
                "GitHub".to_string(),
                "https://github.com".to_string(),
            ]);

            println!("\nCăutare 'Rust':");
            let _ = bk.exec(&["search".to_string(), "Rust".to_string()]);

            terminal.register(Box::new(bk));
        }
        Err(e) => eprintln!("Eroare BKCommand: {}", e),
    }
    terminal.run();
}
