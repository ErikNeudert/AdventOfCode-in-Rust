use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("res/day6_1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {}", e)
        });
    let race_sheet: RaceSheet = parse_input(Box::new(lines));
    let races: Vec<(usize, usize)> = get_race_tuples(race_sheet);

    let mut result_sum = 1;
    for (time, distance) in races {
        let range = calculate_winning_range((time, distance));
        //plus one as start and end is included
        println!("range: {:?}", range);
        let count_winning_options = range.1 - range.0 + 1; 
        println!(" - winning options: {}", count_winning_options);
        result_sum *= count_winning_options; 
    }

    println!("result: {}", result_sum);

    Ok(())
}

struct RaceSheet {
    times: Vec<usize>,
    distances: Vec<usize>
}

fn get_race_tuples(race_sheet: RaceSheet) -> Vec<(usize, usize)> {
    if race_sheet.times.len() != race_sheet.distances.len() {
        panic!("non matching time/distance lengths");
    }
    race_sheet.times.into_iter()
        .zip(race_sheet.distances.into_iter())
        .collect()
}

fn calculate_winning_range(race: (usize, usize)) -> (usize, usize) {
    // (time - root(sqr(time) - 4 * min_distance)) / 2 //round up for start
    // (time + root(sqr(time) - 4 * min_distance)) / 2 //round down for end
    let (time, distance) = race;
    let time = time as f64;
    // the distance to cover at least
    let min_distance = (distance + 1) as f64;

    //round up for start
    let from = (time - (time.powi(2) - (4 as f64) * min_distance).sqrt()) / (2 as f64);
    //round down for end
    let to = (time + (time.powi(2) - (4 as f64) * min_distance).sqrt()) / (2 as f64);

    (from.ceil() as usize, to.floor() as usize)
}

// again via ranges, 
//   check which button-press time (incresing the speed by 1 each milli)
//   results in a time > 9

/*
    Time 7
    Required Distance > 9
    time ranges: x = seconds loaded, y = distance traveled
    13. . . . . . . .
    12. . . x x . . .
    11. . . x x . . .
    10. . x x x x . .
    9 . . x x x x . .
    8 . . x x x x . .
    7 . . x x x x . .
    6 . x x x x x x .
    5 . x x x x x x .
    4 . x x x x x x .
    3 . x x x x x x .
    2 . x x x x x x .
    1 . x x x x x x .
    0 x x x x x x x x
      0 1 2 3 4 5 6 7

    y = f(x) -> x * (time-x) = min_distance

    x * (7 - x) = 9 |
    7x + -x*x = 9
    7x + -x^2 = 9       | -9 : move 9 to left side
    7x + -x^2 - 9 = 0   | no idea why i can reverse the signs? * -1 maybe? yep
    x^2 - 7x + 9 = 0    | insert the non-linear function into a formula for solving for range:
                        | (-B + root(B^2 - 4A * C)) / 2A  where A=1, B=-7, C=9
    
    (time + root(sqr(time) - 4 * min_distance)) / 2
    (time - root(sqr(time) - 4 * min_distance)) / 2


    distance > 9 when pressing for 2 - 5ms = 4 ways to win
    
    Time 15
    Required Distance > 40
    Win range: 4-11

    formula:
    distance
    

    
    
*/
fn parse_input(mut iterator: Box<dyn Iterator<Item=String>>) -> RaceSheet {
    let times: Vec<usize> = iterator.next().expect("'Time' line missing")
        .strip_prefix("Time:")
        .expect("should start with 'Time:'")
        .split_whitespace()
        .into_iter()
        .map(|nr| nr.parse::<usize>().expect("Could not parse nr."))
        .collect();
    let distances: Vec<usize> = iterator.next().expect("'Distance' line missing")
        .strip_prefix("Distance:")
        .expect("should start with 'Distance:'")
        .split_whitespace()
        .into_iter()
        .map(|nr| nr.parse::<usize>().expect("Could not parse nr."))
        .collect();

    RaceSheet {
        times: times,
        distances: distances
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_calculate_winning_range() {
        let range = calculate_winning_range((7, 9));
        assert_eq!((2, 5), range);
        let range = calculate_winning_range((15, 40));
        assert_eq!((4, 11), range);
        //has to be bigger than the distance!, 
        //e.g. just add 1 to the distance to beat, to have a min-distance
        let range = calculate_winning_range((30, 200)); 
        assert_eq!((11, 19), range);
    }

    #[test]
    fn test_calculate_range() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200";
        let lines = Box::new(input.split("\n")
            .map(|line| line.to_string()));
        let sheet: RaceSheet = parse_input(lines);

        let tuples: Vec<(usize, usize)> = get_race_tuples(sheet);
        assert_eq!(vec![(7, 9), (15, 40), (30, 200)], tuples);
    }

    #[test]
    fn test_parse_input() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200";
        let lines = Box::new(input.split("\n")
            .map(|line| line.to_string()));
        let sheet: RaceSheet = parse_input(lines);

        assert_eq!(vec![7, 15, 30], sheet.times);
        assert_eq!(vec![9, 40, 200], sheet.distances);
    }
}