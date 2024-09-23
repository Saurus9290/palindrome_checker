use std::io::{self, Write}; // Import for handling user input

// Function to check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    // Clean the string by converting to lowercase and removing non-alphanumeric characters
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric()) // Keep only alphanumeric characters
        .map(|c| c.to_lowercase().to_string()) // Convert to lowercase
        .collect(); // Collect into a new String
    
    // Check if the cleaned string is equal to its reverse
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    loop {
        // Prompt the user to input a string
        print!("Enter a string to check if it's a palindrome (or type 'exit' to quit): ");
        io::stdout().flush().unwrap(); // Flush to display the prompt

        // Read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Trim the input and exit the loop if the user types 'exit'
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Check if the input string is a palindrome
        if is_palindrome(input) {
            println!("\"{}\" is a palindrome!", input);
        } else {
            println!("\"{}\" is not a palindrome.", input);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome("A man, a plan, a canal, Panama"));
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("Madam"));
        assert!(is_palindrome("12321"));
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("Not a palindrome"));
    }
}
