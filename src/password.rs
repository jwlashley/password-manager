use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn generate_password(length: i32, want_upper: bool, want_number: bool, want_symbol: bool) -> String {
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