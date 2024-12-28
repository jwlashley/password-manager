// Import necessary modules from the standard library and external crates
use std::io::{self, Write};  // For handling input/output operations
use rand::seq::SliceRandom;  // Provides tools for randomly selecting items from collections
use rand::thread_rng;        // Provides a random number generator

// Function that prompts user for yes/no input and returns a boolean
// Takes a string slice as the prompt message
fn get_yes_or_no(prompt: &str) -> bool {
    // Print the prompt without a newline and ensure it's displayed immediately
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    
    // Create a new empty String to store user input
    let mut input = String::new();
    
    // Read a line of input from the user into our string
    // expect() will panic with the given message if reading fails
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    
    // Check if the first character (after trimming whitespace) matches 'y' or 'Y'
    // Returns true for yes, false for any other input
    matches!(input.trim().chars().next(), Some('y') | Some('Y'))
}

// Function that generates a password based on user preferences
// Takes password length and boolean flags for different character types
fn generate_password(length: i32, want_upper: bool, want_number: bool, want_symbol: bool) -> String {
    // Create vectors containing our different character sets
    let lowercase_characters: Vec<char> = ('a'..='z').collect();
    let uppercase_characters: Vec<char> = ('A'..='Z').collect();
    let numbers: Vec<char> = ('0'..='9').collect();
    let symbols: Vec<char> = String::from("!@#$%^&*?").chars().collect();
    
    // Create an empty vector to store all allowed characters
    let mut selected_characters: Vec<char> = Vec::new();
    
    // Lowercase letters are always included
    let want_lower = true;
    if want_lower {
        selected_characters.extend(lowercase_characters.iter().clone());
    }
    
    // Add other character types based on user preferences
    if want_upper {
        selected_characters.extend(uppercase_characters.iter().clone());
    }
    if want_number {
        selected_characters.extend(numbers.iter().clone());
    }
    if want_symbol {
        selected_characters.extend(symbols.iter().clone());
    }
    
    // Create a random number generator
    let mut rng = thread_rng();
    
    // Generate the password:
    // 1. Create a range from 0 to desired length
    // 2. Map each number to a randomly chosen character from our pool
    // 3. Collect these characters into a String
    let password: String = (0..length)
        .map(|_| *selected_characters.choose(&mut rng).unwrap())
        .collect();
    
    // Return the generated password
    password
}

#[derive(Debug)] // Allows us to print the struct
// Account type
struct Account{
    service_name: String,
    username: String,
    password: String,
}

impl Account{
    fn new(service: String, user: String, pass: String) -> Self {
        Self {
            service_name: service,
            username: user,
            password: pass,
        }
    }
}

// Main function - entry point of the program
fn main() {
    let mut service: String = String::new();
    print!("Enter the Service this account is for: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut service)
        .expect("Failed to read input.");
    service = service.trim().to_string();



    let mut username = String::new();
    print!("Enter your username: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read input.");
    username = username.trim().to_string();
    // Set a fixed password length
    let password_length: i32 = 12;
    
    // Get user preferences for character types
    let use_uppercase = get_yes_or_no("Include Uppercase Letters? (y/n): ");
    println!("Uppercase letters will {}", if use_uppercase {"be included."} else {"not be included."});
    
    let use_numbers = get_yes_or_no("Include Numbers? (y/n): ");
    println!("Numbers letters will {}", if use_numbers {"be included."} else {"not be included."});
    
    let use_symbols = get_yes_or_no("Include Symbols? (y/n): ");
    println!("Symbols letters will {}", if use_symbols {"be included."} else {"not be included."});
    
    // Generate password using collected preferences
    let password_options = generate_password(password_length, use_uppercase, use_numbers, use_symbols);
    
    //Create account object
    let account = Account::new(service ,username, password_options);

    // Display the generated password
    println!("Service name: {}", account.service_name);
    println!("Username: {}", account.username);
    println!("Password: {}", account.password);

}
