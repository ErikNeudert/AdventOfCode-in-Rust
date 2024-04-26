use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn run() -> std::io::Result<()> {
    let file = File::open("res/day7_1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {}", e)
        });


    let mut hands: Vec<(Hand, usize)> = parse_lines(Box::new(lines));
    sort_hands_asc(&mut hands);
    //max rank = number of hands
    //define weakness of hand
    //weakest gets rank 1
    //rank * bid = winnings

    let sum: usize = hands.into_iter().enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum();

    println!("{:?}", sum);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    typ: Typ, 
    cards: [usize; 5]//Vec<u8>
}

//named typ to avoid type the keyword as var name 
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Typ {
    //ordering is important for derived ordering, HighCard is the lowest=first element
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn sort_hands_asc(hands: &mut Vec<(Hand, usize)>) {
    //ignore the bids
    //order asc (a to b), desc is b to a
    hands.sort_by(|a, b| a.0.cmp(&b.0));
}

// Memory brute force: 
// nr accesses: 1
// memory: 55dim array, 5^5=3125 * (5 cards + 1 Typ)
//     all possibilities in a matrix of theoretical size:
//         4bit required for 13 card enum possiblities,
//         3bit for 7 Typ possibilities
//         5cards*4bit + 1type*3bit = 23bit
//         3125matrix points * 23bit = 9375+62500 =71875bit ~ 71kb
// 5^5 is only possible, if I find a way to shrink the 13 card possibilities to a range of 5
pub const TYP_MATRIX: [[[[[Typ; 5]; 5]; 5]; 5]; 5] = initialize_typ_matrix();
//lets go memory brute force
//no init time, due to const fn!! calculate at compile time
pub const fn initialize_typ_matrix() -> [[[[[Typ; 5]; 5]; 5]; 5]; 5] {
    let mut typ_matrix = [[[[[Typ::HighCard; 5]; 5]; 5]; 5]; 5];
    
    let mut i0: usize = 0;
    while i0 <= 4 {
        let mut i1: usize = 0;
        while i1 <= 4 {
            let mut i2: usize = 0;
            while i2 <= 4 {
                let mut i3: usize = 0;
                while i3 <= 4 {
                    let mut i4: usize = 0;
                    while i4 <= 4 {
                        //this is basically the same approach as just calculating the hand on the fly xD
                        let typ = identify_hand_type([i0, i1, i2, i3, i4]);
                        typ_matrix[i0][i1][i2][i3][i4] = typ;                        
                        i4 += 1;
                    }
                    i3 += 1;
                }
                i2 += 1;
            }
            i1 += 1;
        }
        i0 += 1;
    }

    typ_matrix
}

/// 
/// identify type based on the card similarity
/// 
/// variants is the count of possible different cards passed
pub const fn identify_hand_type(cards: [usize; 5]) -> Typ {
    identify_hand_type_with_variants(cards, [0; 5])
}
pub const fn identify_hand_type_with_variants<const V: usize>(cards: [usize; 5], mut occurrences: [usize; V]) -> Typ {
    //card to occurrence count mapping
    let mut i = 0;
    while i < 5 {
        let card = cards[i];
        occurrences[card] += 1;
        i += 1;
    }

    let mut has_pair = false;
    let mut has_three = false;
    let mut i = 0;
    while i < occurrences.len() {
        let occurrence = occurrences[i];
        //does the compiler optimize to shortcut occurrence == 0?
        if occurrence == 5 {
            return Typ::FiveOfAKind;
        }
        if occurrence == 4 {
            return Typ::FourOfAKind;
        }
        if occurrence == 3 {
            if has_pair {
                return Typ::FullHouse;
            } else {
                has_three = true;
            }
        }
        if occurrence == 2 {
            if has_pair {
                return Typ::TwoPair;
            } else if has_three {
                return Typ::FullHouse;
            } else {
                has_pair = true;
            }
        }
        i += 1;
    }
    if has_three {
        return Typ::ThreeOfAKind;
    } else if has_pair {
        return Typ::OnePair;
    }
    return Typ::HighCard;
}

fn to_card(char: char) -> usize {
    match char {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        '9' => 7,
        '8' => 6,
        '7' => 5,
        '6' => 4,
        '5' => 3,
        '4' => 2,
        '3' => 1,
        '2' => 0,
        _ => panic!("Unhandled Card char: {}", char)
    }
}

fn parse_lines(lines: Box<dyn Iterator<Item=String>>) -> Vec<(Hand, usize)> {
     lines.map(|line| parse_line(line))
        .collect()
}

// 32T3K 765
fn parse_line(line: String) -> (Hand, usize) {
    let (hand, bid) = match line.split_once(' ') {
        Some(tuple) => tuple,
        None => panic!("line should contain exactly one blank space")
    };

    let cards: [usize; 5] = hand.chars()
        .map(|c| to_card(c))
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();

    let cards_reduced_range = reduce_variant_range(cards);

    let typ = TYP_MATRIX[cards_reduced_range[0]]
                        [cards_reduced_range[1]]
                        [cards_reduced_range[2]]
                        [cards_reduced_range[3]]
                        [cards_reduced_range[4]];
    let hand = Hand {
        typ: typ,
        cards: cards
    };
    let bid = bid.parse::<usize>().expect("Should be a positive integer");

    (hand, bid)
}

/// Bench Results:
pub fn reduce_variant_range(cards: [usize; 5]) -> [usize; 5] {
    //init with 8, which is > the max real value of 4
    let mut variant_map = [8 as usize; 13];
    let mut result = [0 as usize; 5];
    //first is always 0, result[0] therefor also 0
    variant_map[cards[0]] = 0;

    result[1] = match variant_map[cards[1]] {
        8 => {
            variant_map[cards[1]] = 1;
            1
        },
        _ => variant_map[cards[1]]
    };
    result[2] = match variant_map[cards[2]] {
        8 => {
            variant_map[cards[2]] = 2;
            2
        },
        _ => variant_map[cards[2]]
    };
    result[3] = match variant_map[cards[3]] {
        8 => {
            variant_map[cards[3]] = 3;
            3
        },
        _ => variant_map[cards[3]]
    };
    result[4] = match variant_map[cards[4]] {
        8 => {
            variant_map[cards[4]] = 4;
            4
        },
        _ => variant_map[cards[4]]
    };
    return result;
}

#[cfg(test)]
mod tests {
    use crate::day7_1::{*};
    use std::cmp::Ordering;

    #[test]
    fn test_cmp_hands() {
        let hand1 = Hand {
            typ: Typ::HighCard,
            cards: [0, 1, 2, 3, 4]
        };
        let hand2 = Hand {
            typ: Typ::HighCard,
            cards: [0, 1, 2, 3, 4]
        };
        assert_eq!(Ordering::Equal, hand1.cmp(&hand1));
        assert_eq!(Ordering::Equal, hand1.cmp(&hand2));
        let hand2 = Hand {
            typ: Typ::HighCard,
            cards: [1, 0, 2, 3, 4]
        };
        assert_eq!(Ordering::Less, hand1.cmp(&hand2));
        let hand2 = Hand {
            typ: Typ::OnePair,
            cards: [1, 1, 2, 3, 4]
        };
        assert_eq!(Ordering::Less, hand1.cmp(&hand2));
        let hand2 = Hand {
            typ: Typ::OnePair,
            cards: [0, 0, 2, 3, 4]
        };
        assert_eq!(Ordering::Less, hand1.cmp(&hand2));
        let hand1 = Hand {
            typ: Typ::FourOfAKind,
            cards: [0, 0, 0, 0, 2]
        };
        let hand2 = Hand {
            typ: Typ::FourOfAKind,
            cards: [0, 0, 0, 0, 1]
        };
        assert_eq!(Ordering::Greater, hand1.cmp(&hand2));
    }

    #[test]
    fn test_sort_hands_asc() {
        let input = "32T3K 765\n\
                     T55J5 684\n\
                     KK677 28\n\
                     KTJJT 220\n\
                     QQQJA 483";
        let lines = Box::new(input.split("\n")
            .map(|line| line.to_string()));

        /*
        So, the first step is to put the hands in order of strength:

            32T3K is the only one pair and the other hands are all a stronger type, 
                so it gets rank 1.
            KK677 and KTJJT are both two pair. Their first cards both have the 
                same label, but the second card of KK677 is stronger (K vs T), 
                so KTJJT gets rank 2 and KK677 gets rank 3.
            T55J5 and QQQJA are both three of a kind. QQQJA has 
                a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
         */
        let mut hands = parse_lines(lines);
        sort_hands_asc(&mut hands);
        //compare bids
        assert_eq!(765, hands[0].1);
        assert_eq!(220, hands[1].1);
        assert_eq!(28, hands[2].1);
        assert_eq!(684, hands[3].1);
        assert_eq!(483, hands[4].1);
    }
    
    #[test]
    fn test_reduce_variant_range() {
        // input == output
        assert_eq!([0, 0, 0, 0, 0], reduce_variant_range([0, 0, 0, 0, 0]));
        assert_eq!([0, 0, 0, 0, 4], reduce_variant_range([0, 0, 0, 0, 1]));
        assert_eq!([0, 0, 0, 3, 3], reduce_variant_range([0, 0, 0, 1, 1]));
        assert_eq!([0, 0, 0, 3, 4], reduce_variant_range([0, 0, 0, 1, 2]));
        assert_eq!([0, 0, 2, 2, 4], reduce_variant_range([0, 0, 1, 1, 2]));
        assert_eq!([0, 0, 2, 3, 4], reduce_variant_range([0, 0, 1, 2, 3]));
        assert_eq!([0, 1, 2, 3, 4], reduce_variant_range([0, 1, 2, 3, 4]));

        //with actual different ranges
        assert_eq!([0, 0, 0, 0, 0], reduce_variant_range([12, 12, 12, 12, 12]));
        assert_eq!([0, 0, 2, 0, 0], reduce_variant_range([3, 3, 1, 3, 3]));
        assert_eq!([0, 1, 1, 1, 0], reduce_variant_range([7, 0, 0, 0, 7]));
        assert_eq!([0, 1, 2, 2, 2], reduce_variant_range([3, 0, 2, 2, 2]));
        assert_eq!([0, 0, 2, 2, 4], reduce_variant_range([0, 0, 11, 11, 12]));
        assert_eq!([0, 0, 2, 3, 4], reduce_variant_range([0, 0, 7, 8, 4]));
        assert_eq!([0, 1, 2, 3, 4], reduce_variant_range([8, 9, 10, 11, 12]));
    }

    #[test]
    fn test_initialize_typ_matrix() {
        let typ_matrix = initialize_typ_matrix();

        assert_eq!(Typ::FiveOfAKind, typ_matrix[0][0][0][0][0]);
        assert_eq!(Typ::FourOfAKind, typ_matrix[0][0][0][0][1]);
        assert_eq!(Typ::FullHouse, typ_matrix[0][0][0][1][1]);
        assert_eq!(Typ::ThreeOfAKind, typ_matrix[0][0][0][1][2]);
        assert_eq!(Typ::TwoPair, typ_matrix[0][0][1][1][2]);
        assert_eq!(Typ::OnePair, typ_matrix[0][0][1][2][3]);
        assert_eq!(Typ::HighCard, typ_matrix[0][1][2][3][4]);
    }
    #[test]
    fn test_identify_hand_type() {
        //test with max 5 variants
        assert_eq!(Typ::FiveOfAKind, identify_hand_type([0, 0, 0, 0, 0]));
        assert_eq!(Typ::FourOfAKind, identify_hand_type([0, 0, 0, 0, 1]));
        assert_eq!(Typ::FullHouse, identify_hand_type([0, 0, 0, 1, 1]));
        assert_eq!(Typ::ThreeOfAKind, identify_hand_type([0, 0, 0, 1, 2]));
        assert_eq!(Typ::TwoPair, identify_hand_type([0, 0, 1, 1, 2]));
        assert_eq!(Typ::OnePair, identify_hand_type([0, 0, 1, 2, 3]));
        assert_eq!(Typ::HighCard, identify_hand_type([0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_parse_line() {
        let input = "32T3K 765";
        let (hand, bid): (Hand, usize) = parse_line(input.to_string());

        assert_eq!(Typ::OnePair, hand.typ);
        assert_eq!([1, 0, 8, 1, 11], hand.cards);
        assert_eq!(765, bid);
    }
}