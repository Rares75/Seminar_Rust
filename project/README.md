# TPaste - Command Output Sharing Service

A lightweight, multi-user pastebin service written in Rust. Share command outputs securely with unique paste codes and token-based authentication.

## 🎯 Overview

TPaste is a distributed pastebin application consisting of a server and client component. It allows users to:

- **Capture command outputs** and save them to the server
- **Share pastes** via unique codes
- **Authenticate securely** with username/password and token-based sessions
- **Retrieve pastes** created by other users
- **Automatic token-based login** for returning users

## 🏗️ Architecture

### Components

```
├── tpaste-server/          # TCP server handling authentication and paste storage
│   ├── src/
│   │   ├── main.rs         # Main server loop, connection handling
│   │   ├── db_functions.rs # Database operations (SQLite)
│   │   ├── db_model.rs     # Data structures (User, Paste, Token)
│   │   ├── helper_funcions.rs # Utilities (password hashing, code generation)
│   │   └── login.rs        # Placeholder login functions
│   └── Cargo.toml          # Server dependencies
│
└── tpaste-cli/             # TCP client for user interaction
    ├── src/
    │   └── main.rs         # Client implementation with pipe support
    └── Cargo.toml          # Client dependencies
```

### Technology Stack

- **Language**: Rust
- **Protocol**: Raw TCP with newline-delimited messages
- **Database**: SQLite with bcrypt password hashing
- **Authentication**: Username/password + session tokens
- **Concurrency**: Thread-per-connection model

## 📋 Features

### User Authentication

- User registration with username validation (3-30 characters, alphanumeric + underscore)
- Secure password hashing using bcrypt
- Session tokens with 60-day expiration
- Automatic token-based login

### Paste Management

- Save command outputs with unique 10-character codes
- Query pastes by code
- View paste metadata (author, creation date)
- Automatic cascade deletion when user is deleted

### Client Features

- Interactive CLI interface
- Pipe support: `command | tpaste`
- Token-based session persistence
- Help command for available operations

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (Edition 2024 or adjust Cargo.toml)
- Linux/macOS/Windows with TCP networking support

### Building

```bash
# Build both server and client
cargo build --release

# Or build individually
cd tpaste-server && cargo build --release
cd tpaste-cli && cargo build --release
```

### Running

#### Server

```bash
cd tpaste-server
./target/release/tpaste-server
# Server starts on 127.0.0.1:8080
# Creates tpaste.db in current directory
```

#### Client

```bash
cd tpaste-cli
./target/release/tpast-cli
# Connect to server and start entering commands
```

## 📖 Usage Guide

### Client Commands

#### Registration

```
tpaste> sign_up
Enter username: myuser
Enter password: mypassword
OK: Account created and logged in.
```

#### Login

```
tpaste> login
Enter username: myuser
Enter password: mypassword
OK: Login successful! TOKEN:abc123...
Session saved locally.
```

#### Save Command Output

```
tpaste> echo "Hello, World!" | tpaste
SUCCESS: Paste created! Code: sl6DqnGHEX
```

#### Retrieve a Paste

```
tpaste> link:sl6DqnGHEX
Author: myuser
Date: 2026-01-08 10:30:45
Content: Hello, World!
```

#### Available Commands

```
tpaste> help
Available commands:
  tpaste <command> - Save command output to paste
  exit - Disconnect
  help - Show this message
```

### Direct Pipe Usage

```bash
# Send output directly from pipe without interactive CLI
ls -la | ./tpast-cli
whoami | ./tpast-cli
cat file.txt | ./tpast-cli
```

### Automatic Token Login

- Token is saved to `.tpaste_token` in client directory
- On next connection, client automatically logs in if valid token exists
- Session persists across client restarts (within 60-day expiration)

## 🔌 Protocol Specification

### Message Format

All messages are newline-delimited (`\n`)

### Authentication Messages

**Sign Up:**

```
sign_up
username
password
```

Response:

```
OK: Account created and logged in.
```

**Login:**

```
login
username
password
```

Response:

```
OK: Login successful! TOKEN:token_string
```

**Token Login:**

```
token
token_string
```

Response:

```
OK: Welcome back via token!
```

### Paste Messages

**Create Paste:**

```
echo "content" | tpaste
```

Response:

```
SUCCESS: Paste created! Code: code123ABC
```

**Retrieve Paste:**

```
link:code123ABC
```

Response:

```
Author: username
Date: 2026-01-08 10:30:45
Content: paste content here
```

## 💾 Database Schema

### Users Table

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);
```

### Pastes Table

```sql
CREATE TABLE pastes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    code TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

### Tokens Table

```sql
CREATE TABLE tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

### Indexes

- `idx_pastes_code` - Fast paste lookup by code
- `idx_tokens_token` - Fast token validation

## 🔐 Security Features

- **Password Hashing**: bcrypt with configurable cost (default 12)
- **Username Lowercasing**: Prevents impersonation attacks
- **Token Expiration**: 60-day sliding window per login
- **SQL Injection Prevention**: Parameterized queries throughout
- **Input Validation**: Username and password constraints enforced

## 🛠️ Configuration

### Server

Edit connection address in `src/main.rs`:

```rust
TcpListener::bind("127.0.0.1:8080").unwrap();
```

### Database

Database file location: `tpaste.db` (created in server's working directory)

### Token Expiration

Edit token lifetime in `db_functions.rs`:

```rust
let expires_at = created_at + Duration::days(60);
```

## 📦 Dependencies

### Server

- `rusqlite` 0.38.0 - SQLite database driver
- `bcrypt` 0.17.1 - Password hashing
- `chrono` 0.4 - Date/time handling
- `serde` / `serde_json` - JSON serialization
- `serde_derive` - Derive macros

### Client

- `rusqlite` 0.38.0 - Database support

## ⚠️ Current Limitations & Known Issues

1. **Edition Mismatch**: Cargo.toml specifies edition 2024 (not yet released). Change to 2021:

   ```toml
   edition = "2021"
   ```

2. **Unused Imports**: 32 compiler warnings for unused imports/functions

   - Consider removing unused database methods (`read_token_from_file`, `save_token_to_file`)
   - Clean up unused imports in each module

3. **No Rate Limiting**: No protection against spam/abuse
4. **Plain TCP**: No encryption; use in secure networks only
5. **Hardcoded Address**: Server bound to localhost only
6. **Single Machine**: Token-based auth only works with single server instance

## 🚧 Future Improvements

- [ ] Fix Cargo.toml edition to 2021
- [ ] Remove all unused imports and functions
- [ ] Add TLS/encryption support
- [ ] Implement rate limiting
- [ ] Add paste expiration mechanism
- [ ] Support distributed token validation
- [ ] Implement paste deletion by owner
- [ ] Add paste search functionality
- [ ] Web UI for paste sharing
- [ ] Docker containerization
- [ ] Configuration file support
- [ ] Metrics and logging
- [ ] Comprehensive test suite

## 📝 File Structure

```
project/
├── README.md                          # This file
├── tpaste-server/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                   # 245 lines - Server entry point & connection handler
│   │   ├── db_functions.rs           # 282 lines - Database operations
│   │   ├── db_model.rs               # 28 lines - Data models
│   │   ├── helper_funcions.rs        # 112 lines - Utility functions
│   │   └── login.rs                  # 14 lines - Login stubs
│   └── target/
│       └── debug/tpaste-server       # Compiled binary
│
├── tpaste-cli/
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs                   # 107 lines - Client implementation
│   └── target/
│       └── debug/tpast-cli           # Compiled binary
│
└── users.txt                          # (unused)
```

## 🧪 Testing

### Quick Test Flow

```bash
# Terminal 1 - Start server
cd tpaste-server
./target/debug/tpaste-server

# Terminal 2 - Run client
cd tpaste-cli
./target/debug/tpast-cli

# In client:
sign_up
testuser
password123

ls -la | tpaste
# Get the code: abc123XYZ

link:abc123XYZ
# See paste content with metadata
```

### Pipe Test

```bash
echo "test content" | ./tpast-cli
```

## 📄 License

This is a Rust seminar project. No specific license defined.

## 👤 Author

Seminar Rust Project (2026)

---

**Last Updated**: January 8, 2026
**Status**: Working (32 compiler warnings - unused code to clean up)
