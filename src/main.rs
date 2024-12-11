use std::{collections::HashMap, io};

fn main() {
    println!("Enter a list of numbers separated by spaces:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("You entered: {:?}", numbers);

    if let Some(mode) = find_mode(&numbers) {
        println!("The mode is: {}", mode)
    } else {
        println!("No mode found (empty input or all numbers are unique).")
    }

    if let Some(median) = find_median(&mut numbers) {
        println!("The median is: {}", median)
    } else {
        println!("No median found (empty input).")
    }
}

fn find_median(nums: &mut [i32]) -> Option<f64> {
    if nums.is_empty() {
        return None;
    }

    nums.sort();

    let len = nums.len();
    if len % 2 == 0 {
        let mid1 = nums[len / 2 - 1];
        let mid2 = nums[len / 2];
        Some((mid1 + mid2) as f64 / 2.0)
    } else {
        Some(nums[len / 2] as f64)
    }
}

fn find_mode(nums: &[i32]) -> Option<i32> {
    let mut frequency_map = HashMap::new();

    for &num in nums {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    frequency_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
}
