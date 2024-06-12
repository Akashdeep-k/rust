use std::io::{self, Write};

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = std::cmp::max(num, current_sum + num);
        max_sum = std::cmp::max(max_sum, current_sum);
    }

    max_sum
}

fn main() {
    print!("Enter an array of integers (comma-separated): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let array: Vec<i32> = input.trim()
        .split(',')
        .map(|s| s.trim().parse().expect("Please enter valid integers"))
        .collect();

    println!("Maximum subarray sum: {}", max_subarray_sum(&array));
}
