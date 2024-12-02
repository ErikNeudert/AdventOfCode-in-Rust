use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::y2023::day2_1::Game;

fn main() -> std::io::Result<()> {
    let file = File::open("res/y2023/day2.2.txt")?;
    let reader = BufReader::new(file);

    //add up the IDs of the games that would have been possible, you get 8.
    let mut sum_of_game_powers = 0;
    for maybe_line in reader.lines() {
        let line = maybe_line?;
        let game = Game::from(&*line);

        //find the maximum number for each color,
        //as this is the number required to play the game, 
        //less would not allow the set to be played
        let min_red = game.sets_of_cubes.iter()
            .map(|set| set.red)
            .max().unwrap_or(0);
        let min_green = game.sets_of_cubes.iter()
            .map(|set| set.green)
            .max().unwrap_or(0);
        let min_blue = game.sets_of_cubes.iter()
            .map(|set| set.blue)
            .max().unwrap_or(0);

        let game_power = min_red * min_green * min_blue;
        sum_of_game_powers += game_power;
    }
    println!("{}", sum_of_game_powers);

    Ok(())
}
