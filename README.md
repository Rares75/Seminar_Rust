# Rust Seminar Exercises & Final Project

**Author:** Cabalau Rares (B2)  
**Course:** Rust Programming Seminar - Faculty of Computer Science

This repository contains all exercises completed during a Rust programming seminar at the faculty, plus a comprehensive final project demonstrating advanced Rust concepts.

---

## 📚 Repository Structure

### Lab Exercises (Weeks 1-7)

These are structured programming exercises covering fundamental to intermediate Rust concepts:

#### [Lab 01](lab01/) - Basic Functions & Number Theory

- **Topics:** Functions, conditionals, loops, prime number detection
- **Problems:**
  - Prime number validation
  - GCD (Greatest Common Divisor) computation
  - String manipulation and formatting

#### [Lab 02](lab02/) - String Operations & Ownership

- **Topics:** String handling, ownership, borrowing, mutable references
- **Problems:**
  - Character insertion and string concatenation
  - Integer to string conversion
  - Space and formatting utilities

#### [Lab 04](lab04/) - File I/O & Text Processing

- **Topics:** File operations, string iteration, text analysis
- **Problems:**
  - Finding longest lines (in bytes vs UTF-8 characters)
  - String reversal and palindrome checking
  - Text file analysis

#### [Lab 05](lab05/) - Structs & Data Structures

- **Topics:** Struct definition, data organization, file parsing
- **Problems:**
  - Student record management (age, name, phone number)
  - Finding youngest/oldest student
  - File-based data parsing

#### [Lab 06](lab06/) - Traits, SQLite Database & Command Terminal

- **Topics:** Trait-based design patterns, SQLite database operations, interactive CLI
- **Problems:**
  - Implementing custom terminal with pluggable commands
  - Command trait system (Ping, Count, Times, Hello commands)
  - Database integration with SQLite
  - Interactive shell-like interface

#### [Lab 07](lab07/) - Operator Overloading & Complex Numbers

- **Topics:** Trait implementations (Add, Sub, Mul, Neg), generic programming, mathematical operations
- **Problems:**
  - Complex number struct with arithmetic operations
  - Operator overloading for mathematical operations
  - Generic parameter handling with type conversions

---

## 🎯 Final Project

### [TPaste - Command Output Sharing Service](project/)

A full-featured, production-grade pastebin service for sharing command outputs securely.

**Architecture:** Distributed TCP-based server-client application with SQLite database

#### Components

- **[tpaste-server](project/tpaste-server/)** - Multi-threaded TCP server

  - User authentication with bcrypt password hashing
  - 60-day session token management
  - Paste storage and retrieval
  - Command routing and database operations

- **[tpaste-cli](project/tpaste-cli/)** - Interactive command-line client
  - User registration and login
  - Pipe support for capturing command output
  - Token-based session persistence
  - Paste history management

#### Key Features

✅ **User Authentication**

- Username/password registration with validation
- Bcrypt password hashing for security
- 60-day session tokens with automatic renewal
- Automatic token-based reconnection

✅ **Paste Management**

- Unique 10-character alphanumeric paste codes
- Paste retrieval with metadata (author, timestamp)
- User-specific paste history
- Cascade deletion on user removal

✅ **Multi-Client Support**

- Thread-per-connection model for concurrent handling
- User isolation and data privacy
- Comprehensive error handling

✅ **Technology Stack**

- Rust Edition 2021
- SQLite with rusqlite
- bcrypt for password hashing
- Raw TCP protocol with newline-delimited messages

---

## 🚀 How to Build & Run

### Prerequisites

- Rust 1.70+ (Edition 2021)
- Cargo package manager
- Linux/macOS/Windows with TCP support

### Building the Project

```bash
# Build all exercises and the final project
cargo build --release

# Or build specific components
cd lab01 && cargo build --release
cd lab06 && cargo build --release
cd project/tpaste-server && cargo build --release
cd project/tpaste-cli && cargo build --release
```

### Running Labs

```bash
cd lab01 && cargo run --release
cd lab02 && cargo run --release
# ... etc for other labs
```

### Running the Final Project

```bash
# Terminal 1: Start the server
cd project/tpaste-server
cargo run --release

# Terminal 2: Start the client
cd project/tpaste-cli
cargo run --release

# Or pipe command output directly
echo "Hello from my command" | cargo run --release
```

---

## 📖 Learning Progression

This repository demonstrates learning progression through Rust:

1. **Labs 1-2:** Core language fundamentals (functions, ownership, borrowing)
2. **Labs 4-5:** Real-world I/O operations and data structures
3. **Lab 6:** Advanced trait patterns and database integration
4. **Lab 7:** Generic programming and operator overloading
5. **Final Project:** Integration of all concepts into a production application

---

## 📝 Notes

Each lab directory contains:

- `Cargo.toml` - Project manifest with dependencies
- `src/main.rs` - Implementation files
- `README.md` - Exercise-specific documentation (where available)
- Test data files (where needed)

The final project includes comprehensive documentation and a full SQLite schema with proper foreign key constraints.

---

**Status:** ✅ All labs completed | ✅ Final project completed
