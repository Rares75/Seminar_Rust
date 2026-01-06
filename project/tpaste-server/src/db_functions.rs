use chrono::{DateTime, Duration, Utc};
use rusqlite::{Connection, Result, params};

use crate::db_model::{Paste, Token, User};
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
    pub fn get_user(&self, username: &str) -> Result<User> {
        let mut stmt = self
            .conn
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
        let mut stmt = self
            .conn
            .prepare("SELECT id,username,password_hash,created_at from users WHERE id=?1")?;

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
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM users WHERE username=?1")?;
        let count: i64 = stmt.query_row(params![username], |row| row.get(0))?;

        Ok(count > 0)
    }

    //token table operation
    pub fn create_token(&self, user_id: i64, token: &str) -> Result<i64> {
        let created_at = Utc::now();
        let expires_at = created_at + Duration::days(60);

        self.conn.execute(
            "INSERT INTO auth_tokens(user_id,token,created_at,expires_at) VALUES (?1,?2,?3,?4)",
            params![
                user_id,
                token,
                created_at.to_rfc3339(),
                expires_at.to_rfc3339()
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    //validate a token
    pub fn validate_token(&self, token: &str) -> Result<Option<i64>> {
        let mut stmt = self
            .conn
            .prepare("SELECT user_id,expires_at FROM auth_token=?1")?;

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

        self.conn.execute(
            "INSERT INTO pastes (user_id,code,content,created_at) VALUES (?1,?2,?3,?4)",
            params![user_id, code, content, created_at],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    //getting the paste by code
    pub fn get_paste_by_code(&self, code: &str) -> Result<Paste> {
        let mut stmt = self
            .conn
            .prepare("SELECT id,user_id,code,content,created_at FROM pastes WHERE code=?1")?;

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
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM pastes WHERE code=?1")?;
        let count: i64 = stmt.query_row(params![code], |row| row.get(0))?;

        Ok(count > 0)
    }
}
