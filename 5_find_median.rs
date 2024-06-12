use std::io::{self, Write};

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
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

    println!("Median: {}", median(&array));
}
