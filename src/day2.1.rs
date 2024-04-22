use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    id: u32,
    sets_of_cubes: Vec<SetOfCube>
}
#[derive(Debug, PartialEq)]
struct SetOfCube {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        if let Some((game_id_str, sets)) = line.split_once(':') {
            let game_id = match game_id_str[5..].parse::<u32>() {
                Ok(id) => id,
                Err(_) => panic!("{}", format!("Could not parse number! {game_id_str} starting at index 5"))
            };
            let sets_of_cubes = sets.split(';').map(|set| SetOfCube::from(set)).collect();
            return Game {
                id: game_id,
                sets_of_cubes: sets_of_cubes
            };
        } else {
            panic!("{}", format!("Invalid input! {line}"));
        }
    }
}

impl From<&str> for SetOfCube {
    //example input:
    //3 blue, 4 red
    fn from(input: &str) -> Self {
        //map each color to its count
        let count_per_color: HashMap<&str, u32> = input.split(',')
            .map(|count_color| {
                let (count, color) = match count_color.trim().split_once(' ') {
                    Some((count, color)) => (count, color),
                    None => panic!("{}", format!("Could not parse color: {count_color}"))
                };
                //parse the u32
                let count = match count.parse::<u32>() {
                    Ok(count) => count,
                    Err(_) => panic!("{}", format!("Could not parse number! {count}"))
                };
                return (color, count);
        }).collect(); //collect as a map
        
        // println!("{:?}", count_per_color);
        SetOfCube { 
            red: *count_per_color.get("red").unwrap_or(&0),
            green: *count_per_color.get("green").unwrap_or(&0),
            blue: *count_per_color.get("blue").unwrap_or(&0),
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("res/day2.1.txt")?;
    let reader = BufReader::new(file);
    //12 red cubes, 13 green cubes, and 14 blue cubes
    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;
    //add up the IDs of the games that would have been possible, you get 8.
    let mut sum = 0;
    'next_game: for maybe_line in reader.lines() {
        let line = maybe_line?;
        let game = Game::from(&*line);

        for set in game.sets_of_cubes {
            if (set.red > max_red_cubes) 
                || (set.green > max_green_cubes) 
                || (set.blue > max_blue_cubes) {
                continue 'next_game;
            }
        }
        
        sum += game.id;
    }
    println!("{}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_create_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from(input);
        assert_eq!(game.id, 1);
        assert_eq!(game.sets_of_cubes[0], SetOfCube { red: 4, green: 0, blue: 3 });
        assert_eq!(game.sets_of_cubes[1], SetOfCube { red: 1, green: 2, blue: 6 });
        assert_eq!(game.sets_of_cubes[2], SetOfCube { red: 0, green: 2, blue: 0 });
    }
    #[test]
    fn test_from_setofcubes() {
        let input = "3 blue, 4 red, 1 green";
        let set_of_cubes = SetOfCube::from(input);
        assert_eq!(set_of_cubes, SetOfCube { red: 4, green: 1, blue: 3 });
    }
}