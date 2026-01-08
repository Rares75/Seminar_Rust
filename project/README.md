# TPaste - Command Output Sharing Service

A lightweight, multi-user pastebin service written in Rust. Share command outputs securely with unique paste codes and token-based authentication.

## 🎯 Overview

TPaste is a distributed pastebin application consisting of a server and client component. It allows users to:

- **Capture command outputs** via shell pipes and save them to the server
- **Share pastes** via unique 10-character codes
- **Authenticate securely** with username/password and 60-day session tokens
- **Retrieve pastes** created by other users by paste code
- **Automatic token-based login** for returning users with local token persistence
- **View paste history** from your account

## 🏗️ Architecture

### Components

```
├── tpaste-server/          # TCP server handling authentication and paste storage
│   ├── src/
│   │   ├── main.rs         # Main server loop, connection handling, command routing
│   │   ├── db_functions.rs # Database operations (SQLite) - CRUD operations
│   │   ├── db_model.rs     # Data structures (User, Paste, Token)
│   │   └── helper_funcions.rs # Utilities (password validation, code/token generation)
│   └── Cargo.toml          # Server dependencies
│
└── tpaste-cli/             # TCP client for user interaction
    ├── src/
    │   └── main.rs         # Interactive CLI with automatic token login and pipe support
    └── Cargo.toml          # Client dependencies
```

### Technology Stack

- **Language**: Rust (Edition 2021)
- **Protocol**: Raw TCP with newline-delimited messages
- **Database**: SQLite with bcrypt password hashing
- **Authentication**: Username/password + 60-day session tokens
- **Concurrency**: Thread-per-connection model for handling multiple clients
- **Dependencies**: rusqlite, bcrypt, chrono, serde

## 📋 Features

### User Authentication

- **User registration** with strict validation:
  - Usernames: 3-30 characters, alphanumeric + underscore only
  - Passwords: 6-100 characters
  - Duplicate username prevention
- **Secure password storage** using bcrypt hashing
- **Session tokens** with 60-day expiration
- **Automatic token-based login** on reconnection (token saved locally)
- **Token validation** with expiration checking

### Paste Management

- **Save command outputs** with unique 10-character random codes (alphanumeric)
- **Retrieve pastes** by code with author and timestamp information
- **View paste history** - list all pastes created by current user
- **Paste metadata** - author username, creation date, and full content
- **Cascade deletion** - pastes automatically deleted when user is deleted
- **Combine stdout and stderr** in saved pastes

### Client Features

- **Interactive CLI** prompt interface (`tpaste>`)
- **Pipe support** - capture command output directly: `command | tpaste`
- **Automatic token persistence** - session tokens saved to `.tpaste_token`
- **Auto-login on startup** - uses saved token if valid
- **Help command** - view available commands
- **Graceful disconnection** - `exit` or `quit` commands

### Server Features

- **Multi-client support** - handles concurrent connections with thread-per-connection model
- **User isolation** - users can only see their own paste history
- **Database consistency** - foreign keys with cascade deletion
- **Optimized queries** - indexed lookups on paste codes and tokens
- **Error handling** - comprehensive error messages for all operations

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (Edition 2021)
- Linux/macOS/Windows with TCP networking support
- SQLite3 (usually included or downloaded automatically by rusqlite)

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
cargo run --release
# Or: ./target/release/tpaste-server
# Server listens on 127.0.0.1:8080
# Creates tpaste.db in current directory
# Output: "Server started"
```

#### Client

```bash
cd tpaste-cli
cargo run --release
# Or: ./target/release/tpast-cli
# Automatically logs in with saved token if available
# Otherwise prompts for authentication
# Shows: "Connected to tpaste server. Type 'help' for commands."
```

## 📖 Usage Guide

### Authentication Flow

#### First-time Registration

```
tpaste> sign_up
Enter username: john_doe
Enter password: SecurePass123
OK: Account created and logged in.
you are logged in,now you have accest to tpaste server
```

#### Login with Credentials

```
tpaste> login
Enter username: john_doe
Enter password: SecurePass123
OK: Login successful! TOKEN:a1b2c3d4e5f6...
Session saved locally.
```

#### Automatic Token Login

```
# On reconnection (if .tpaste_token exists):
Automatically logged in via token.
```

### Working with Pastes

#### Save Command Output (with Pipe)

```bash
$ echo "Hello, World!" | tpaste
Message saved with code: aBcDeFgHiJ,your user id: 1
```

Or from interactive CLI:

```
tpaste> echo "System info:" | tpaste
Message saved with code: xYz1aB2cD3,your user id: 1
```

#### Retrieve a Paste by Code

```
tpaste> link:xYz1aB2cD3
Author: john_doe
Date: 2025-01-08 15:30:45
Content: System info: Linux kernel 5.15.0...
```

#### View Your Paste History

```
tpaste> my_pastes
Code: aBcDeFgHiJ | Date: 2025-01-08 14:22:10
Code: xYz1aB2cD3 | Date: 2025-01-08 15:30:45
```

#### Get Help

```
tpaste> help
Availble commands are link:code,my_pastes and | tpaste
```

#### Disconnect

```
tpaste> exit
Goodbye!
```


## 🔌 Protocol Specification

### Message Format

All messages are newline-delimited (`\n`). The protocol is text-based and simple.

### Authentication Messages

**Sign Up Request:**
```
sign_up
username
password
```

**Login Request:**
```
login
username
password
```

**Token Login Request:**
```
token
token_string
```

**Server Responses:**
```
OK: Account created and logged in.
OK: Login successful! TOKEN:token_string
OK: Welcome back via token!
ERR: Username invalid: reason
ERR: Username already exists
ERR: Login failed: invalid credentials
ERR: Token invalid or expired.
```

### Command Messages

**Paste Creation (via pipe):**
```
command_output | tpaste
```

**Paste Retrieval:**
```
link:paste_code
```

**View User Pastes:**
```
my_pastes
```

**Get Help:**
```
help
```

**Disconnect:**
```
exit
```

## 🗄️ Database Schema

### Users Table
```sql
CREATE TABLE users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);
```

### Pastes Table
```sql
CREATE TABLE pastes(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    code TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE INDEX idx_pastes_code ON pastes(code);
```

### Tokens Table
```sql
CREATE TABLE tokens(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE INDEX idx_tokens_token ON tokens(token);
```

## 🔒 Security Features

- **Bcrypt hashing** - passwords hashed with bcrypt before storage
- **Token expiration** - tokens expire after 60 days and are validated on each use
- **SQL injection prevention** - uses parameterized queries throughout
- **Password validation** - enforces minimum length (6 chars) and maximum (100 chars)
- **Username validation** - alphanumeric + underscore only, 3-30 character limit
- **User isolation** - users can only access their own paste history
- **Secure random tokens** - 32-character hex tokens generated from system time and PID

## 🛠️ Implementation Details

### Code Generation Algorithm

Paste codes are 10-character random alphanumeric strings generated using:
- System timestamp as seed
- Linear congruential pseudo-random number generator
- Character set: `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`

### Token Generation Algorithm

Auth tokens are 32-character hex strings generated using:
- System timestamp hash
- Process ID hash
- Double hashing for additional entropy

### Output Capture

The server captures both `stdout` and `stderr` from executed commands:
```rust
let stdout = String::from_utf8_lossy(&output.stdout);
let stderr = String::from_utf8_lossy(&output.stderr);
let content = format!("{}{}", stdout, stderr);
```

## 📝 Complete Workflow Example

```bash
# Terminal 1 - Start server
cd tpaste-server
cargo run --release
# Output: Server started

# Terminal 2 - Client registration and usage
cd tpaste-cli
cargo run --release
# Output: Connected to tpaste server...

tpaste> sign_up
Enter username: alice
Enter password: MySecurePass123
OK: Account created and logged in.
you are logged in,now you have accest to tpaste server

# Save some command output
tpaste> whoami | tpaste
Message saved with code: Kx9mNpQrSt,your user id: 1

# Exit and reconnect (will auto-login)
tpaste> exit
Goodbye!

# Reconnect
cargo run --release
Automatically logged in via token.

# Retrieve the paste
tpaste> link:Kx9mNpQrSt
Author: alice
Date: 2026-01-08 10:15:30
Content: alice
```

## 🐛 Known Limitations

- Server runs on hardcoded address `127.0.0.1:8080` (not configurable)
- Username comparison is case-insensitive (stored as-is but checked lowercase)
- No encryption on the wire (consider TLS for production)
- SQLite is single-file, suitable for small deployments
- Thread-per-connection model not optimal for many concurrent users

## 🚀 Future Improvements

- [ ] TLS/SSL encryption for wire security
- [ ] Configuration file support (port, bind address, database path)
- [ ] User account deletion endpoint
- [ ] Paste expiration and auto-cleanup
- [ ] Bulk paste download
- [ ] Web interface alongside CLI
- [ ] Rate limiting
- [ ] Audit logging
