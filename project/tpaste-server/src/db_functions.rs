use crate::db_model::{Paste, Token, User};

use crate::{hash_password, validate_password, validate_username};
use bcrypt::verify;
use chrono::{DateTime, Duration, Utc};
use rusqlite::{Connection, Result, params};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
fn read_from_stream(stream: &mut TcpStream) -> String {
    let mut buf = [0; 512];
    let n = stream.read(&mut buf).unwrap_or(0);
    String::from_utf8_lossy(&buf[..n]).trim().to_string()
}
#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    //open or create the DB
    pub fn new(path: &str) -> Result<Self, rusqlite::Error> {
        let file = Connection::open(path)?;
        let db = Database {
            conn: Arc::new(Mutex::new(file)),
        };
        db.create_tables()?;
        Ok(db)
    }
    //creating necessary tables

    fn create_tables(&self) -> Result<()> {
        //user tables
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL UNIQUE,
        password_hash TEXT NOT NULL,
        created_at TEXT NOT NULL
      )",
            [], //specific for SQLite, this means we don't have any parameters to insert in DB
        )?;
        //Paste table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pastes(
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            user_id INTEGER NOT NULL,
            code TEXT NOT NULL UNIQUE,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE)
            ",
            [],
        )?;
        //creating index in code(Paste) for fast search in DB
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_pastes_code ON pastes(code)",
            [],
        )?;

        //Token table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tokens(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE)",
            [],
        )?;
        //creating index in token(TOKEN) for fast seacth in DB
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tokens_token ON tokens(token)",
            [],
        )?;

        Ok(())
    }
    //User table operations

    //creating a new user
    pub fn create_user(&self, username: &str, password_hash: &str) -> Result<i64> {
        let created_at = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO users (username,password_hash,created_at) VALUES (?1,?2,?3)",
            params![username, password_hash, created_at],
        )?;
        Ok(conn.last_insert_rowid())
    }

    //finding a user
    pub fn get_user(&self, username: &str) -> Result<User> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id,username,password_hash,created_at from users WHERE username=?1")?;

        let user = stmt.query_row(params![username], |row| {
            let created_str: String = row.get(3)?;
            let created_at = DateTime::parse_from_rfc3339(&created_str)
                .unwrap()
                .with_timezone(&Utc);

            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at,
            })
        })?;

        Ok(user)
    }

    pub fn get_user_id(&self, user_id: i64) -> Result<User> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT id,username,password_hash,created_at from users WHERE id=?1")?;

        let user = stmt.query_row(params![user_id], |row| {
            let created_str: String = row.get(3)?;
            let created_at = DateTime::parse_from_rfc3339(&created_str)
                .unwrap()
                .with_timezone(&Utc);

            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                created_at,
            })
        })?;

        Ok(user)
    }
    pub fn username_exists(&self, username: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();

        let lowercase_username = username.to_lowercase();
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username=?1")?;
        let count: i64 = stmt.query_row(params![lowercase_username], |row| row.get(0))?;

        Ok(count > 0)
    }

    //token table operation
    pub fn create_token(&self, user_id: i64, token: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let created_at = Utc::now();
        let expires_at = created_at + Duration::days(60);

        conn.execute(
            "INSERT INTO auth_tokens(user_id,token,created_at,expires_at) VALUES (?1,?2,?3,?4)",
            params![
                user_id,
                token,
                created_at.to_rfc3339(),
                expires_at.to_rfc3339()
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    //validate a token
    pub fn validate_token(&self, token: &str) -> Result<Option<i64>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT user_id,expires_at FROM auth_token=?1")?;

        let result = stmt.query_row(params![token], |row| {
            let user_id: i64 = row.get(0)?;
            let expires_str: String = row.get(1)?;
            let expires_at = DateTime::parse_from_rfc3339(&expires_str)
                .unwrap()
                .with_timezone(&Utc);

            Ok((user_id, expires_at))
        });

        match result {
            Ok((user_id, expires_at)) => {
                //check if the token expired
                if Utc::now() > expires_at {
                    Ok(None) //expired token
                } else {
                    Ok(Some(user_id)) //validate token
                }
            }
            Err(e) => Ok(None), //token doesn't exist
        }
    }

    //paste table operations
    pub fn create_paste(&self, user_id: i64, code: &str, content: &str) -> Result<i64> {
        let created_at = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO pastes (user_id,code,content,created_at) VALUES (?1,?2,?3,?4)",
            params![user_id, code, content, created_at],
        )?;

        Ok(conn.last_insert_rowid())
    }

    //getting the paste by code
    pub fn get_paste_by_code(&self, code: &str) -> Result<Paste> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT id,user_id,code,content,created_at FROM pastes WHERE code=?1")?;

        let paste = stmt.query_row(params![code], |row| {
            let created_str: String = row.get(4)?;
            let created_at = DateTime::parse_from_rfc3339(&created_str)
                .unwrap()
                .with_timezone(&Utc);

            Ok(Paste {
                id: row.get(0)?,
                user_id: row.get(1)?,
                code: row.get(2)?,
                content: row.get(3)?,
                created_at,
            })
        })?;

        Ok(paste)
    }

    //check if a code already exists
    pub fn code_exists(&self, code: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM pastes WHERE code=?1")?;
        let count: i64 = stmt.query_row(params![code], |row| row.get(0))?;

        Ok(count > 0)
    }
    pub fn sign_up(&self, username: String, password: String) -> Result<i64, String> {
        //lowercase the username to prevent impersonation attack
        let username = username.to_lowercase();

        //hashing the password and save the user in the DB
        let hashed = hash_password(&password).unwrap();
        let id: i64 = self
            .create_user(&username, &hashed)
            .map_err(|e| e.to_string())?;

        Ok(id)
    }
    pub fn login(&self, username: String, password: String) -> Result<i64, String> {
        //make the username with lowercase
        let lowercase_username: String = username.to_lowercase();

        match self.get_user(&lowercase_username) {
            Ok(check_user) => match verify(password, &check_user.password_hash) {
                Ok(true) => Ok(check_user.id.expect("Error at getting the user id")),
                Ok(false) => Err("Wrong Username or password!".to_string()),
                Err(e) => Err(format!("Crypto error: {}", e)),
            },
            Err(e) => Err("Wrong Username or password!".to_string()),
        }
    }
}
