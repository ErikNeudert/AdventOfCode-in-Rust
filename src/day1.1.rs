use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("res/day1.1.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for maybe_line in reader.lines() {
        let line = maybe_line?;
        //parse first number
        let first_number = findFirstNumber(line.chars());
        let last_number = findFirstNumber(line.chars().rev());
        let combined_number = first_number * 10 + last_number;
        sum += combined_number;
    }
    println!("{}", sum);

    Ok(())
}

fn findFirstNumber(input: impl Iterator<Item=char>) -> i32 {
    for char in input {
        if char >= '0' && char <= '9' {
            return char as i32 - '0' as i32;
        }
    }

    panic!("No number found in input");
}