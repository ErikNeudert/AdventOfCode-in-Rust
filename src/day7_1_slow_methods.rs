use crate::day7_1::*;

//following the slower versions of card reduction and hand determination for comparison:

// Brute force: 
// nr accesses: 4 + 3 + 2 + 1 = 10
// memory: 3bit counter * 5 cards = 15bit
// compare card 1 to 2-5, note 
// let typ = identify_hand_type(cards, cards_reduced_range);

//memory approach is fastest :)

//this has basically the same performance as the full static version
pub fn reduce_variant_range_half_static(cards: [usize; 5]) -> [usize; 5] {
    //init with 8, which is > the max real value of 4
    let mut variant_map = [8 as usize; 13];
    let mut result = [0 as usize; 5];
    //first is always 0, result[0] therefor also 0
    variant_map[cards[0]] = 0;

    for i in 1..5 {
        result[i] = match variant_map[cards[i]] {
            8 => {
                variant_map[cards[i]] = i;
                i
            },
            _ => variant_map[cards[i]]
        };
    }

    return result;
}

/// reduces given values to the max 5 different usizes possible in cards, e.g. to a range of 5
/// 
/// Bench Results:
/// Not sure why this takes 5 times longer
pub fn reduce_variant_range_slow(cards: [usize; 5]) -> [usize; 5] {
    //init with 8, which is > the max real value of 4
    let mut variant_map = [8 as usize; 13];
    let mut counter = 0;
    for source_id in cards {
        let target_id = variant_map[source_id];
        if target_id == 8 {
            //not mapped yet
            // target_id = counter; //why is this not needed, what was my idea?
            variant_map[source_id] = counter;
            counter += 1;
        }
    }

    cards.into_iter()
        .map(|c| variant_map[c]) //map to reduced range equivalent
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

pub const fn identify_hand_type13(cards: [usize; 5]) -> Typ {
    let mut occurrences = [0 as usize; 13];
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
    while i < 13 {
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

#[cfg(test)]
mod tests {
    // use crate::*;
    use crate::day7_1_slow_methods::{*};

    #[test]
    fn test_reduce_variant_range_slow() {
        // input == output
        assert_eq!([0, 0, 0, 0, 0], reduce_variant_range_slow([0, 0, 0, 0, 0]));
        assert_eq!([0, 0, 0, 0, 1], reduce_variant_range_slow([0, 0, 0, 0, 1]));
        assert_eq!([0, 0, 0, 1, 1], reduce_variant_range_slow([0, 0, 0, 1, 1]));
        assert_eq!([0, 0, 0, 1, 2], reduce_variant_range_slow([0, 0, 0, 1, 2]));
        assert_eq!([0, 0, 1, 1, 2], reduce_variant_range_slow([0, 0, 1, 1, 2]));
        assert_eq!([0, 0, 1, 2, 3], reduce_variant_range_slow([0, 0, 1, 2, 3]));
        assert_eq!([0, 1, 2, 3, 4], reduce_variant_range_slow([0, 1, 2, 3, 4]));

        //with actual different ranges
        assert_eq!([0, 0, 0, 0, 0], reduce_variant_range_slow([12, 12, 12, 12, 12]));
        assert_eq!([0, 0, 1, 0, 0], reduce_variant_range_slow([3, 3, 1, 3, 3]));
        assert_eq!([0, 1, 1, 1, 0], reduce_variant_range_slow([7, 0, 0, 0, 7]));
        assert_eq!([0, 1, 2, 2, 2], reduce_variant_range_slow([3, 0, 2, 2, 2]));
        assert_eq!([0, 0, 1, 1, 2], reduce_variant_range_slow([0, 0, 11, 11, 12]));
        assert_eq!([0, 0, 1, 2, 3], reduce_variant_range_slow([0, 0, 7, 8, 4]));
        assert_eq!([0, 1, 2, 3, 4], reduce_variant_range_slow([8, 9, 10, 11, 12]));
    }

    #[test]
    fn test_reduce_variant_range_half_static() {
        // input == output
        assert_eq!([0, 0, 0, 0, 0], reduce_variant_range_half_static([0, 0, 0, 0, 0]));
        assert_eq!([0, 0, 0, 0, 4], reduce_variant_range_half_static([0, 0, 0, 0, 1]));
        assert_eq!([0, 0, 0, 3, 3], reduce_variant_range_half_static([0, 0, 0, 1, 1]));
        assert_eq!([0, 0, 0, 3, 4], reduce_variant_range_half_static([0, 0, 0, 1, 2]));
        assert_eq!([0, 0, 2, 2, 4], reduce_variant_range_half_static([0, 0, 1, 1, 2]));
        assert_eq!([0, 0, 2, 3, 4], reduce_variant_range_half_static([0, 0, 1, 2, 3]));
        assert_eq!([0, 1, 2, 3, 4], reduce_variant_range_half_static([0, 1, 2, 3, 4]));

        //with actual different ranges
        assert_eq!([0, 0, 0, 0, 0], reduce_variant_range_half_static([12, 12, 12, 12, 12]));
        assert_eq!([0, 0, 2, 0, 0], reduce_variant_range_half_static([3, 3, 1, 3, 3]));
        assert_eq!([0, 1, 1, 1, 0], reduce_variant_range_half_static([7, 0, 0, 0, 7]));
        assert_eq!([0, 1, 2, 2, 2], reduce_variant_range_half_static([3, 0, 2, 2, 2]));
        assert_eq!([0, 0, 2, 2, 4], reduce_variant_range_half_static([0, 0, 11, 11, 12]));
        assert_eq!([0, 0, 2, 3, 4], reduce_variant_range_half_static([0, 0, 7, 8, 4]));
        assert_eq!([0, 1, 2, 3, 4], reduce_variant_range_half_static([8, 9, 10, 11, 12]));
    }
    
    

    #[test]
    fn test_identify_hand_type13() {
        //test with max 5 variants
        assert_eq!(Typ::FiveOfAKind, identify_hand_type13([0, 0, 0, 0, 0]));
        assert_eq!(Typ::FourOfAKind, identify_hand_type13([0, 0, 0, 0, 1]));
        assert_eq!(Typ::FullHouse, identify_hand_type13([0, 0, 0, 1, 1]));
        assert_eq!(Typ::ThreeOfAKind, identify_hand_type13([0, 0, 0, 1, 2]));
        assert_eq!(Typ::TwoPair, identify_hand_type13([0, 0, 1, 1, 2]));
        assert_eq!(Typ::OnePair, identify_hand_type13([0, 0, 1, 2, 3]));
        assert_eq!(Typ::HighCard, identify_hand_type13([0, 1, 2, 3, 4]));
    }
}