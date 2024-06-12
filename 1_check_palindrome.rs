use std::io::{self, Write};

fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    print!("Enter a string: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    println!("Is \"{}\" a palindrome? {}", input, is_palindrome(input));
}
