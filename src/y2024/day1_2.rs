use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run() -> std::io::Result<()> {
    let file = File::open("res/y2024/day1_1.txt")?;
    let reader = BufReader::new(file);

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

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

    let mut count_right_occurrences  = HashMap::new();
    for num in right_numbers {
        match count_right_occurrences.get(&num) {
            Some(count) => {count_right_occurrences.insert(num, count + 1);}
            None => {count_right_occurrences.insert(num, 1);}
        }
    }

    let mut similarity_score = 0;
    for num in left_numbers {
        let count = match count_right_occurrences.get(&num) {
            Some(count) => count,
            None => &0,
        };

        similarity_score += num * count;
    }

    println!("{}", similarity_score);

    Ok(())
}