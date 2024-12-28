Rust Password Manager
A secure, command-line password manager built in Rust that allows users to generate, encrypt, and store passwords for various services.
Features

Secure password generation with customizable options:

Adjustable password length
Optional uppercase letters
Optional numbers
Optional special characters


AES-256-GCM encryption for stored passwords
Command-line interface for easy interaction
Service-specific account management
Secure memory handling (Rust's memory safety)

Getting Started
Prerequisites

Rust (latest stable version)
Cargo package manager

Installation

Clone the repository:

bashCopygit clone https://github.com/yourusername/rust-password-manager.git
cd rust-password-manager

Build the project:

bashCopycargo build --release

Run the program:

bashCopycargo run
Usage

Enter the service name for the account (e.g., "Gmail", "Twitter")
Enter your username for the service
Choose password generation options:

Include uppercase letters?
Include numbers?
Include special symbols?


The program will generate and store your encrypted password

Security Features

AES-256-GCM encryption for password storage
Unique nonce generation for each encryption
Zero-knowledge architecture (passwords are encrypted before storage)
Memory safety guaranteed by Rust's ownership system

Technical Details

Built in Rust
Uses aes-gcm for encryption
Implements secure random number generation
Command-line interface built with Rust's standard I/O

Future Enhancements

Database integration for persistent storage
Master password implementation
GUI interface
Password strength checking
Import/Export functionality
Secure password sharing
Multi-device sync support

Contributing
Contributions are welcome! Please feel free to submit pull requests.
License
[Add your chosen license here]
Acknowledgments

Rust Cryptography Working Group for the aes-gcm crate
The Rust community for excellent documentation and support

Security Notice
This project is currently in development and should not be used for production password management yet. Always use well-tested, professionally audited password managers for sensitive information.

Built with ❤️ using Rust