use std::io::{self, Write};
use password_manager::account::Account;
use password_manager::database;
use password_manager::password::generate_password;


//Function to get user input
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input.trim().to_string()
}



// Function for converting user answers to Y or N to make a true or false bool.
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

// Main function - entry point of the program
fn main() {
    // Initialize database
    if let Err(e) = database::init_db() {
        println!("Failed to initialize database: {}", e);
        return;
    }

    let create_new = get_yes_or_no("Do you want to create a new account (y/n): ");

    if create_new {
        // Get service name
        let service = get_user_input("Enter the Service this account is for: ");

        // Get username
        let username = get_user_input("Enter your username: ");

        // Password generation preferences
        let password_length: i32 = 12;
        let use_uppercase = get_yes_or_no("Include Uppercase Letters? (y/n): ");
        let use_numbers = get_yes_or_no("Include Numbers? (y/n): ");
        let use_symbols = get_yes_or_no("Include Symbols? (y/n): ");

        // Generate and create account
        let password = generate_password(password_length, use_uppercase, use_numbers, use_symbols);
        let account = Account::new(service, username, password.clone());

        // Show account details
        println!("\nAccount created:");
        println!("Service: {}", account.service_name);
        println!("Username: {}", account.username);
        println!("Generated password: {}", password);

        // Save to database
        match database::save_account(&account) {
            Ok(_) => println!("Account saved successfully!"),
            Err(e) => println!("Failed to save account: {}", e),
        }
    } else {
        let service = get_user_input("Enter the service you want to lookup: ");

        match database::find_accounts_by_service(&service) {
            Ok(accounts) => {
                if accounts.is_empty() {
                    println!("No accounts found for service {}", service);
                } else {
                    println!("Found {} accounts for service {}", accounts.len(), service);
                    for account in accounts {
                        println!("\nUsername: {}", account.username);
                        println!("Password: {}", account.get_password());
                    }
                }
            }
            Err(e) => println!("Error looking up accounts: {}", e),
        }
    }
}