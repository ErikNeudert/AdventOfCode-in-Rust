use std::fs::File;
use std::io::{prelude::*, BufReader};
use linked_hash_set::LinkedHashSet;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file = File::open("res/y2023/day4_1.txt")?;
    let reader = BufReader::new(file);
    
    let cards: Vec<Card> = parse_lines(Box::new(reader.lines()));

    //matching numbers on scratch cards cause the following cards to be duplicated
    //card 1 has 4 matches, so cards 2-5 get another instance.
    //card 2 has 2 matches and exists 2 times, so you get 3 and 4 twice each.
    //card 3 has 2 matches and exists 4 times, so you get 4 and 5 four times.

    //each card exists once at the start!
    let card_counts: Vec<usize> = calculate_cards_counts(cards);
    // for c in card_counts {
    //     println!("{}", c);
    // }
    let sum: usize = card_counts.iter().sum();
    println!("Summed counts: {}", sum);

    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
struct Card {
    id: usize,
    winnings: LinkedHashSet<usize>,
    yours: LinkedHashSet<usize>
}

fn calculate_cards_counts(mut cards: Vec<Card>) -> Vec<usize> {
    //each card exists once at the start
    let mut card_counts: Vec<usize> = vec![1; cards.len()];
    //process each cards number
    //card numbers are processed 0 based
    for card_idx in 0..cards.len() {
        let card = cards.remove(0);
        assert_eq!((card_idx + 1) as usize, card.id);

        let count_of_current_card = card_counts[card_idx];
        let matches = calculate_matches(card);
        //the next n=matches numbers get added 1, for each card of the current index
        for next in 0..matches {
            let next_idx = card_idx + next + 1;
            // println!("card_idx: {}, next: {}, next_idx: {}", card_idx, next, next_idx);
            if next_idx >= card_counts.len() {
                panic!("Incorrect number of cards, trying to access idx {}, but have only {} cards", next_idx, card_counts.len());
            }
            //increment the next cards count by the amount of current card
            card_counts[next_idx] += count_of_current_card;
        }
    }
    card_counts
}

fn calculate_matches(card: Card) -> usize {
     card.yours.into_iter()
        .filter(|num| card.winnings.contains(num))
        .count()
}

fn parse_lines(iterator: Box<dyn Iterator<Item=Result<String, std::io::Error>>>) -> Vec<Card> {
    iterator
        .map(|line| line.expect("Error reading line"))
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Card {
    let (card, nrs) = line.split_once(':').expect("Line should contain a ':'");
    let card_id: usize = card.strip_prefix("Card").expect("Line should start with 'Card '")
        .trim()
        .parse()
        .expect(&format!("'{card}' should have been 'Card x'"));
    let (winning_nrs, your_nrs) = nrs.split_once('|').expect("Right part of string should contain a '|'");
    let winning_numbers: LinkedHashSet<usize> = winning_nrs.split_whitespace()
        .into_iter()
        .map(|str| str.parse::<usize>().expect(&format!("Could not parse {str}")))//&
        .collect();
    let your_numbers: LinkedHashSet<usize> = your_nrs.split_whitespace()
        .into_iter()
        .map(|str| str.parse::<usize>().expect(&format!("Could not parse {str}")))//&
        .collect();
    Card {
        id: card_id, 
        winnings: winning_numbers,
        yours: your_numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cards_counts() {
        let input = "Card 1: 1 2 | 1 2 3\n\
                     Card 2: 2 3 | 3 4 5\n\
                     Card 3: 3 4 | 10 11";
        let lines = Box::new(input.split("\n")
            .map(|str| Ok(str.to_string())));
        let cards = parse_lines(lines);
        let counts = calculate_cards_counts(cards);
        // for c in counts {
        //     println!("{}", c);
        // }
        assert_eq!(1, counts[0]);
        assert_eq!(2, counts[1]);
        assert_eq!(4, counts[2]);
        assert_eq!(7, counts.iter().sum::<usize>());
    }

    #[test]
    fn test_calculate_matches() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_line(input);

        let points = calculate_matches(card);
        assert_eq!(4, points);
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
    fn test_parse_line2() {
        let input = "Card    1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = parse_line(input);
        let expected = Card {
            id: 1,
            winnings: vec![41, 48, 83, 86, 17].into_iter().collect(),
            yours: vec![83, 86,  6, 31, 17,  9, 48, 53].into_iter().collect(),
        };
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