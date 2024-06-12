use std::io::{self, Write};

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    print!("Enter a string: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    println!("Reversed string: {}", reverse_string(input));
}
