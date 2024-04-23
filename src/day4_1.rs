use std::fs::File;
use std::io::{prelude::*, BufReader};
use linked_hash_set::LinkedHashSet;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file = File::open("res/day4_1.txt")?;
    let reader = BufReader::new(file);
    
    let cards: Vec<Card> = parse_lines(Box::new(reader.lines()));

    let point_sum = calculate_points_sum(cards);

    println!("Summed points: {}", point_sum);

    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
struct Card {
    id: u32,
    winnings: LinkedHashSet<u32>,
    yours: LinkedHashSet<u32>
}

fn calculate_points_sum(cards: Vec<Card>) -> u32 {
    cards.into_iter()
        .map(|card| calculate_winning_points(card))
        .fold(0, |acc, num| acc + num)
}

fn calculate_winning_points(card: Card) -> u32 {
    let mut res: u32 = 0;
    for num in card.yours {
        if card.winnings.contains(&num) {
            if res == 0 {
                res = 1;
            } else {
                res *= 2;
            }
        }
    }
    res
}

fn parse_lines(iterator: Box<dyn Iterator<Item=Result<String, std::io::Error>>>) -> Vec<Card> {
    iterator
        .map(|line| line.expect("Error reading line"))
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Card {
    let (card, nrs) = line.split_once(':').expect("Line should contain a ':'");
    let card_id: u32 = card.strip_prefix("Card ").expect("Line should start with 'Card '")
        .parse()
        .expect(&format!("'{card}' should have been 'Card x'"));
    let (winning_nrs, your_nrs) = nrs.split_once('|').expect("Right part of string should contain a '|'");
    let winning_numbers: LinkedHashSet<u32> = winning_nrs.split_whitespace()
        .into_iter()
        .map(|str| str.parse::<u32>().expect(&format!("Could not parse {str}")))//&
        .collect();
    let your_numbers: LinkedHashSet<u32> = your_nrs.split_whitespace()
        .into_iter()
        .map(|str| str.parse::<u32>().expect(&format!("Could not parse {str}")))//&
        .collect();
    Card {
        id: card_id, 
        winnings: winning_numbers,
        yours: your_numbers
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_calculate_points_sum() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let lines = Box::new(input.split("\n")
            .map(|str| Ok(str.to_string())));
        let cards = parse_lines(lines);
        let sum = calculate_points_sum(cards);
        assert_eq!(10, sum);
    }
    #[test]
    fn test_calculate_winnings() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_line(input);

        let points = calculate_winning_points(card);
        assert_eq!(8, points);
    }
    #[test]
    fn test_parse_lines() {
        let input = "Card 1: 1 | 2\n\
                     Card 2: 3 | 4";
        let lines = Box::new(input.split("\n")
            .map(|str| Ok(str.to_string())));
         
        let actual = parse_lines(lines);
        let expected = vec![
            Card {id: 1,
                winnings: vec![1].into_iter().collect(),
                yours: vec![2].into_iter().collect()},
            Card {id: 2,
                winnings: vec![3].into_iter().collect(),
                yours: vec![4].into_iter().collect() }
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_line() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = parse_line(input);
        let expected = Card {
            id: 1,
            winnings: vec![41, 48, 83, 86, 17].into_iter().collect(),
            yours: vec![83, 86,  6, 31, 17,  9, 48, 53].into_iter().collect(),
        };
        assert_eq!(expected, actual);
    }
}