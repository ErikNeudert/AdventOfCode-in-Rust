use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> std::io::Result<()> {
    let file = File::open("res/y2024/day2_1.txt")?;
    let reader = BufReader::new(file);

    let mut safe_reports = 0;
    // let mut reports = Vec::new();
    for report in reader.lines() {
        let levels: Vec<i32> = report.unwrap().split_whitespace()
            .map(|lvl: &str| lvl.parse::<i32>().unwrap())
            .collect();

        //safe if:
        // - The levels are either all increasing or all decreasing.
        // - Any two adjacent levels differ by at least one and at most three.
        let mut is_increasing = false;
        let mut is_safe = true;
        for i in 1..levels.len() {
            let previous = levels[i - 1];
            let current = levels[i];
            let difference = (previous - current).abs();
            if difference < 1 || difference > 3 {
                is_safe = false;
                break;
            }
            
            if i == 1 {
                is_increasing = previous < current;
                continue;
            }

            if is_increasing != (previous < current) {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);

    Ok(())
}