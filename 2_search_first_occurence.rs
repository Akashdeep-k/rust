use std::io::{self, Write};

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    print!("Enter a sorted array of integers (comma-separated): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let array: Vec<i32> = input.trim()
        .split(',')
        .map(|s| s.trim().parse().expect("Please enter valid integers"))
        .collect();

    print!("Enter the target number: ");
    io::stdout().flush().unwrap();
    let mut target_input = String::new();
    io::stdin().read_line(&mut target_input).expect("Failed to read line");
    let target: i32 = target_input.trim().parse().expect("Please enter a valid integer");

    println!("First occurrence of {}: {:?}", target, first_occurrence(&array, target));
}
