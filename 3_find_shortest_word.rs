use std::io::{self, Write};

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    print!("Enter a string of words: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    println!("The shortest word is: {:?}", shortest_word(input));
}
