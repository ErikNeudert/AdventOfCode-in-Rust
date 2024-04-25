use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("res/day7_1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {}", e)
        });
    let hands: Vec<Hand> = parse_input(Box::new(lines));

    let range = calculate_winning_range(race_sheet.time, race_sheet.distance);
    //plus one as start and end is included
    println!("range: {:?}", range);
    let count_winning_options = range.1 - range.0 + 1; 
    println!(" - winning options: {}", count_winning_options);

    println!("result: {}", count_winning_options);

    Ok(())
}

struct Hand {
    typ: Typ, 
    cards: Vec<Card>
}

//named typ to avoid type the keyword as var name 
enum Typ {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

//lets go memory brute force
impl From<Vec<Card>> for Typ {
    fn from(cards: Vec<Card>) -> Self {
        /* Question: How can I identify types with least card comparisos?
        Brute force: 
            nr accesses: 4 + 3 + 2 + 1 = 10
            memory: 3bit counter * 5 cards = 15bit
            compare card 1 to 2-5, note 
            
        Memory brute force: 
            nr accesses: 1
            memory: 55dim array, 5^5=3125 * (5 cards + 1 Typ)
                all possibilities in a matrix of theoretical size:
                    4bit required for 13 card enum possiblities,
                    3bit for 7 Typ possibilities
                    5cards*4bit + 1type*3bit = 23bit
                    3125matrix points * 23bit = 9375+62500 =71875bit ~ 71kb
         */
    }
}

enum Card {
    A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2 //if uncompilable, use x9 or c9 for numbers
}

impl From<char> for Card {
    fn from(char: char) -> Self {
        match char {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2
            _ => panic!("Unhandled Card char: {}", char)
        }
    }
}

// 32T3K 765
fn parse_line(line: String) -> Hand {
    let (hand, bid) = match line.split_once(' ') {
        Some(tuple) => tuple
        None => panic!("line should contain exactly one blank space")
    };

    let cards: Vec<Card> = hand.chars()
        .map(|c| Card::from(c))
        .collect();

    //type from cards

    RaceSheet {
        time: time,
        distance: distance
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_line() {
        let input = "32T3K 765";
        let hand: Hand = parse_line(input);

        assert_eq!(71530, sheet.time);
    }
}