use chrono::{DateTime, Duration, Utc};
use rusqlite::{Connection, Result, params};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Database {
    conn: Connection,
}

impl Database {
    //open or creat the DB
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?; //probabil va trebui schimbat cu Arc<Mutex<Connection>>
        let db = Database { conn };
        db.create_tables()?;
        Ok(db)
    }
    //creating necessary tables

    fn create_tables(&self) -> Result<()> {
        //user tables
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL UNIQUE,
        password_hash TEXT NOT NULL,
        created_at TEXT NOT NULL
      )",
            [], //specific for SQLite, this means we don't have any parameters to insert in DB
        )?;
        //Paste table
        self.conn.execute(
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
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_pastes_code ON pastes(code)",
            [],
        )?;

        //Token table
        self.conn.execute(
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
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tokens_token ON tokens(token)",
            [],
        )?;

        Ok(())
    }
    //User table operations

    //creating a new user
    pub fn create_user(&self, username: &str, password_hash: &str) -> Result<i64> {
        let created_at = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO users (username,password_hash,created_at) VALUES (?1,?2,?3)",
            params![username, password_hash, created_at],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    //finding a user
    pub fn get_user(&self, username: &str) -> Result<User> {}
}
