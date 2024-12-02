use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("res/y2023/day6_1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {}", e)
        });
    let race_sheet: RaceSheet = parse_input(Box::new(lines));

    let range = calculate_winning_range(race_sheet.time, race_sheet.distance);
    //plus one as start and end is included
    println!("range: {:?}", range);
    let count_winning_options = range.1 - range.0 + 1; 
    println!(" - winning options: {}", count_winning_options);

    println!("result: {}", count_winning_options);

    Ok(())
}

struct RaceSheet {
    time: usize,
    distance: usize
}

fn calculate_winning_range(time: usize, distance: usize) -> (usize, usize) {
    // (time - root(sqr(time) - 4 * min_distance)) / 2 //round up for start
    // (time + root(sqr(time) - 4 * min_distance)) / 2 //round down for end
    let time = time as f64;

    // the distance to cover at least, therefor + 1
    let min_distance = (distance + 1) as f64;

    //round up for start
    let from = (time - (time.powi(2) - (4 as f64) * min_distance).sqrt()) / (2 as f64);
    //round down for end
    let to = (time + (time.powi(2) - (4 as f64) * min_distance).sqrt()) / (2 as f64);

    (from.ceil() as usize, to.floor() as usize)
}

fn parse_input(mut iterator: Box<dyn Iterator<Item=String>>) -> RaceSheet {
    let time: usize = iterator.next().expect("'Time' line missing")
        .strip_prefix("Time:")
        .expect("should start with 'Time:'")
        //remove whitespaces, as it's actually a single race
        .replace(" ", "")
        .parse::<usize>().expect("Could not parse nr.");
    let distance: usize = iterator.next().expect("'Distance' line missing")
        .strip_prefix("Distance:")
        .expect("should start with 'Distance:'")
        //remove whitespaces, as it's actually a single race
        .replace(" ", "")
        .parse::<usize>().expect("Could not parse nr.");

    RaceSheet {
        time: time,
        distance: distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_winning_range() {
        let range = calculate_winning_range(7, 9);
        assert_eq!((2, 5), range);
        let range = calculate_winning_range(15, 40);
        assert_eq!((4, 11), range);
        //has to be bigger than the distance!, 
        //e.g. just add 1 to the distance to beat, to have a min-distance
        let range = calculate_winning_range(30, 200);
        assert_eq!((11, 19), range);

        let range = calculate_winning_range(71530, 940200);
        assert_eq!((14, 71516), range);
    }

    #[test]
    fn test_parse_input() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200";
        let lines = Box::new(input.split("\n")
            .map(|line| line.to_string()));
        let sheet: RaceSheet = parse_input(lines);

        assert_eq!(71530, sheet.time);
        assert_eq!(940200, sheet.distance);
    }
}