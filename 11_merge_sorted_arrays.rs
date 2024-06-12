use std::io::{self, Write};

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = vec![];
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    if i < arr1.len() {
        merged.extend_from_slice(&arr1[i..]);
    }

    if j < arr2.len() {
        merged.extend_from_slice(&arr2[j..]);
    }

    merged
}

fn main() {
    print!("Enter the first sorted array (comma-separated): ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let arr1: Vec<i32> = input1.trim()
        .split(',')
        .map(|s| s.trim().parse().expect("Please enter valid integers"))
        .collect();

    print!("Enter the second sorted array (comma-separated): ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let arr2: Vec<i32> = input2.trim()
        .split(',')
        .map(|s| s.trim().parse().expect("Please enter valid integers"))
        .collect();

    println!("Merged array: {:?}", merge_sorted_arrays(&arr1, &arr2));
}
