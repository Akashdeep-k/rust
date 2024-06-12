use std::io::{self, Write};

fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    arr.sort();
    arr.get(k - 1).copied()
}

fn main() {
    print!("Enter an array of integers (comma-separated): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut array: Vec<i32> = input.trim()
        .split(',')
        .map(|s| s.trim().parse().expect("Please enter valid integers"))
        .collect();

    print!("Enter the value of k: ");
    io::stdout().flush().unwrap();
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Please enter a valid number");

    println!("The {}-th smallest element is: {:?}", k, kth_smallest(&mut array, k));
}
