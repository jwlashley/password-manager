# Rust Password Manager

A secure, command-line password manager built in Rust that allows users to generate, encrypt, and store passwords for various services. This project implements a modular architecture with SQLite storage and AES-256-GCM encryption.

## Features

- Secure password generation with customizable options:
  - Adjustable password length
  - Optional uppercase letters
  - Optional numbers
  - Optional special characters
- AES-256-GCM encryption for stored passwords
- SQLite database for persistent storage
- Command-line interface for easy interaction
- Service-specific account management
- Secure memory handling (Rust's memory safety)
- Modular architecture for maintainability

## Project Structure

```
src/
├── main.rs           # Entry point, CLI interface
├── account.rs        # Account struct and its implementations
├── password.rs       # Password generation logic
├── database.rs       # Database operations
├── encryption.rs     # Encryption/decryption logic
└── lib.rs           # Module declarations and public exports
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager
- SQLite (included via rusqlite)

### Dependencies

```toml
[dependencies]
rusqlite = "0.29.0"    # SQLite database
aes-gcm = "0.10.2"     # Encryption
rand = "0.8.5"         # Random number generation
```

### Installation

1. Clone the repository:
```bash
git clone https://github.com/jwlashley/password-manager.git
cd password-manager
```

2. Build the project:
```bash
cargo build --release
```

3. Run the program:
```bash
cargo run
```

## Usage

1. Enter the service name for the account (e.g., "Gmail", "Twitter")
2. Enter your username for the service
3. Choose password generation options:
   - Include uppercase letters?
   - Include numbers?
   - Include special symbols?
4. The program will generate, encrypt, and store your password
5. Retrieve passwords by looking up the service name

## Security Features

- AES-256-GCM encryption for password storage
- Unique nonce generation for each encryption
- Zero-knowledge architecture (passwords are encrypted before storage)
- Memory safety guaranteed by Rust's ownership system
- Secure database storage using SQLite
- Modular security implementation for better code review and testing

## Technical Details

- Built in Rust with a modular architecture
- Uses `aes-gcm` for encryption with unique nonces
- SQLite database for persistent storage via `rusqlite`
- Implements secure random number generation
- Command-line interface built with Rust's standard I/O
- Separated concerns for encryption, storage, and password generation

## Future Enhancements

- [x] Database integration for persistent storage
- [x] Modular code architecture
- [ ] Master password implementation
- [ ] GUI interface
- [ ] Password strength checking
- [ ] Import/Export functionality
- [ ] Secure password sharing
- [ ] Multi-device sync support
- [ ] Proper error handling
- [ ] Unit tests for each module

## Contributing

Contributions are welcome! The modular architecture makes it easy to work on individual components. Please feel free to:

- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## Acknowledgments

- Rust Cryptography Working Group for the `aes-gcm` crate
- The Rust community for excellent documentation and support
- SQLite and the `rusqlite` maintainers

## Security Notice

This project is currently in development and should not be used for production password management yet. Always use well-tested, professionally audited password managers for sensitive information.

---
Built with ❤️ using Rust