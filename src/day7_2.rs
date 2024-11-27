use crate::day7_1::{Hand, Typ, sort_hands_asc};
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() -> std::io::Result<()> {
    /*
    1. reorder, J < 2 < 3...
    2. change identify_hand_type, parse J as joker
       just count the jokers,
       and check from best to worst in teh identify hand function,
       counting all J's for each comparison
    */
    let file = File::open("res/day7_1.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {}", e)
        });

    
    //max rank = number of hands
    //define weakness of hand
    //weakest gets rank 1
    //rank * bid = winnings
    
    // print the parsed values and the original line
    // let mut hands = parse_lines_with_line(Box::new(lines));
    // sort_hands_asc_with_line(&mut hands);
    // hands.into_iter()
    //     .for_each(|((hand, bid), line)| 
    //         println!("{:?} {} - {}", hand, bid, line));
        
    let mut hands = parse_lines(Box::new(lines));
    sort_hands_asc(&mut hands);
    let sum: usize = hands.into_iter().enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum();
    println!("{:?}", sum);
    
    Ok(())
}

pub fn sort_hands_asc_with_line(hands: &mut Vec<((Hand, usize), String)>) {
    //ignore the bids
    //order asc (a to b), desc is b to a
    hands.sort_by(|(a, _), (b, _)| a.0.cmp(&b.0));
}

fn to_card(char: char) -> usize {
    match char {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        'J' => 0,
        _ => panic!("Unhandled Card char: {}", char)
    }
}

fn parse_lines(lines: Box<dyn Iterator<Item=String>>) -> Vec<(Hand, usize)> {
    lines.map(|line| parse_line(line))
       .collect()
}

fn parse_lines_with_line(lines: Box<dyn Iterator<Item=String>>) -> Vec<((Hand, usize), String)> {
    lines.map(|line| (parse_line(line.clone()), line))
       .collect()
}

// T55J5 684
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
    //try with dynamic calculation for maintainability
    //could instead increase the size of the TYP_MATRIX to 6 options ^ 5 cards
    let typ = identify_hand_type(cards_reduced_range);

    let hand = Hand {
        typ: typ,
        cards: cards
    };
    let bid = bid.parse::<usize>().expect("Should be a positive integer");

    (hand, bid)
}

/// Replaced static with half-static variant, as it's performance difference is
/// expressable in Centimeters of "light travel velocity".
/// It's around 100 picoseconds ~ 30cm of light
/// while the method takes ~ 15m
/// but way more maintainable
pub fn reduce_variant_range(cards: [usize; 5]) -> [usize; 5] {
    //init with 8, which is > the max real value of 4
    let mut variant_map = [8 as usize; 13];
    let mut result = [0 as usize; 5];
    variant_map[0] = 5; //joker is 5
    
    for i in 0..5 {
        result[i] = match variant_map[cards[i]] {
            // 0 => 5, //map the 0 (J) to the joker (5) which is > the max card 4
            8 => {
                variant_map[cards[i]] = i;
                i
            },
            _ => variant_map[cards[i]]
        };
    }

    return result;
}

//cards can contain a nr'5' meaning Joker.
//joker counts towards all possibilities and evaluates to the best.
pub fn identify_hand_type(cards: [usize; 5]) -> Typ {
    //card to occurrence count mapping
    let mut occurrences = [0 as usize; 6]; //up to 4 normal cards + 1 joker at position 5
    let mut i = 0;
    while i < cards.len() {
        let card = cards[i];
        occurrences[card] += 1;
        i += 1;
    }
    //joker is the 6th extra option in a pattern
    let mut joker_count = occurrences[5];
    occurrences[5] = 0; //reset joker count, so that we can sort the values correctly

    occurrences.sort();
    occurrences.reverse(); //descending, so that we can check the best first

    //make sure best is checked first, including the joker_count
    let mut has_pair = false;
    let mut has_three = false;
    let mut i = 0;
    while i < occurrences.len() -1 /*skip joker*/{
        let occurrence = occurrences[i] + joker_count;
        //does the compiler optimize to shortcut occurrence == 0?
        if occurrence == 5 {
            return Typ::FiveOfAKind;
        }
        if occurrence == 4 {
            return Typ::FourOfAKind;
        }
        if occurrence == 3 {
            if has_pair {
                //two pairs and a joker
                return Typ::FullHouse;
            } else {
                //is this even possible? that two comes before three?
                //does it matter to which pair I add the joker, if two pairs and a joker exist?
                //11J22 
                has_three = true;
                joker_count = 0; //reset joker count, as it's used
            }
        }
        if occurrence == 2 {
            if has_pair {
                return Typ::TwoPair;
            } else if has_three {
                return Typ::FullHouse;
            } else {
                has_pair = true;
                joker_count = 0; //reset joker count, as it's used
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

#[cfg(test)]
mod tests {
    use crate::day7_2::{*};
    use crate::day7_1::{sort_hands_asc, initialize_typ_matrix};
    
    #[test]
    fn test_to_card() { 
        assert!(to_card('J') < to_card('2'));
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

        //new J rule:
        let mut hands = parse_lines(lines);
        assert_eq!(Typ::OnePair, hands[0].0.typ);
        assert_eq!(Typ::FourOfAKind, hands[1].0.typ);
        assert_eq!(Typ::TwoPair, hands[2].0.typ);
        assert_eq!(Typ::FourOfAKind, hands[3].0.typ);
        assert_eq!(Typ::FourOfAKind, hands[4].0.typ);

        assert_eq!(765, hands[0].1); //32T3K
        assert_eq!(684, hands[1].1); //T55J5
        assert_eq!(28, hands[2].1);  //KK677
        assert_eq!(220, hands[3].1); //KTJJT
        assert_eq!(483, hands[4].1); //QQQJA

        sort_hands_asc(&mut hands);
        //compare bids
        assert_eq!(Typ::OnePair, hands[0].0.typ);
        assert_eq!(Typ::TwoPair, hands[1].0.typ);
        assert_eq!(Typ::FourOfAKind, hands[2].0.typ);
        assert_eq!(Typ::FourOfAKind, hands[3].0.typ);
        assert_eq!(Typ::FourOfAKind, hands[4].0.typ);

        assert_eq!(765, hands[0].1); //32T3K
        assert_eq!(28, hands[1].1);  //KK677
        assert_eq!(684, hands[2].1); //T55J5
        assert_eq!(483, hands[3].1); //QQQJA
        assert_eq!(220, hands[4].1); //KTJJT
    }

    #[test]
    fn test_hands_of_example() {
        let input = "QQQQ2 1\n\
                     JKKK2 2";
        let lines = Box::new(input.split("\n")
            .map(|line| line.to_string()));

        //new J rule:
        let mut hands = parse_lines(lines);
        assert_eq!(1, hands[0].1); //QQQQ2
        assert_eq!(2, hands[1].1);  //JKKK2

        sort_hands_asc(&mut hands);
        //compare bids
        assert_eq!(2, hands[0].1);
        assert_eq!(1, hands[1].1);
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
        //joker is 0, everything below J is shifted up by 1
        assert_eq!([2, 1, 9, 2, 11], hand.cards);
        assert_eq!(765, bid);
    }

    #[test]
    fn test_parse_line_with_joker() {
        let input = "T55J5 684";
        let (hand, bid): (Hand, usize) = parse_line(input.to_string());

        assert_eq!(Typ::FourOfAKind, hand.typ);
        assert_eq!([9, 4, 4, 0, 4], hand.cards);
        assert_eq!(684, bid);
    }

    #[test]
    fn test_reduce_variant_range() {
        // input == output
        assert_eq!([5, 5, 5, 5, 5], reduce_variant_range([0, 0, 0, 0, 0]));
        assert_eq!([0, 1, 0, 3, 5], reduce_variant_range([2, 3, 2, 4, 0]));
    }

    #[test]
    fn test_problem_with_full_hourse() {
        // Hand { typ: FullHouse, cards: [0, 2, 9, 2, 12] } 674 - J3T3A 674
        let input = "J3T3A 674";
        let (hand, bid): (Hand, usize) = parse_line(input.to_string());

        assert_eq!(Typ::ThreeOfAKind, hand.typ);
        assert_eq!([0, 2, 9, 2, 12], hand.cards);
        assert_eq!(674, bid);
    }

    #[test]
    fn test_problem_with_full_hourse2() {
        // Hand { typ: FullHouse, cards: [0, 2, 9, 2, 12] } 674 - J3T3A 674
        let input = "KTJJT 674";
        let (hand, bid): (Hand, usize) = parse_line(input.to_string());

        assert_eq!(Typ::FourOfAKind, hand.typ);
        assert_eq!([11, 9, 0, 0, 9], hand.cards);
        assert_eq!(674, bid);
    }

    #[test]
    fn test_problem_with_full_hourse3() {
        // Hand { typ: FullHouse, cards: [0, 1, 6, 6, 11] } 633 - J277K 633
        let input = "J277K 633";
        let (hand, bid): (Hand, usize) = parse_line(input.to_string());

        assert_eq!(Typ::ThreeOfAKind, hand.typ);
        assert_eq!([0, 1, 6, 6, 11], hand.cards);
        assert_eq!(633, bid);
    }

}