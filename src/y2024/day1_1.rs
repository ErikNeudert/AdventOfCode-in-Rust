use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn run() -> std::io::Result<()> {
    let file = File::open("res/y2024/day1_1.txt")?;
    let reader = BufReader::new(file);

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    let mut sum = 0;
    for maybe_line in reader.lines() {
        let line: String = maybe_line.unwrap();

        let all: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let left = all[0].clone();
        let right = all[1].clone();

        // println!("{} - {}", left, right);
        //parse first number
        left_numbers.push(left.parse::<i32>().expect("Expected positive Integer"));
        right_numbers.push(right.parse::<i32>().expect("Expected positive Integer"));
    }

    left_numbers.sort();
    right_numbers.sort();

    for i in 0..left_numbers.len() {
        let left = left_numbers[i];
        let right = right_numbers[i];

        let diff = (left - right).abs();
        sum += diff;
    }

    println!("{}", sum);

    Ok(())
}