use std::io::{self, Write};

fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut prefix = strs[0].to_string();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return "".to_string();
            }
        }
    }
    prefix
}

fn main() {
    print!("Enter a set of strings (comma-separated): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let strings: Vec<&str> = input.trim().split(',').map(|s| s.trim()).collect();

    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}
