// use crate::day7_1::{Hand, Typ};
use crate::y2023::day7_1::Typ;

/*
git commit -m "Day 7.1: trying 7bit matrix access

- recognized that the Matrix could be 7bit addressable, refactoring it therefor
Perfromance was measured on low energy mode of my laptop.
- previous performance: 
  - init:
  - access: 
- bit shift performance: 
  - init:
  - access: "
*/

// Memory brute force: 
// nr accesses: 1
// memory: 55dim array, 5^5=3125 * (5 cards + 1 Typ)
//     all possibilities in a matrix of theoretical size:
//         4bit required for 13 card enum possiblities,
//         3bit for 7 Typ possibilities
//         5cards*4bit + 1type*3bit = 23bit
//         3125matrix points * 23bit = 9375+62500 =71875bit ~ 71kb
// 5^5 is only possible, if I find a way to shrink the 13 card possibilities to a range of 5
// removed 1 dimension, as the first is always 0
pub const TYP_MATRIX_BIT_SHIFT_LEN: usize = 0b100_11_10_1 + 1;
pub const TYP_MATRIX_BIT_SHIFT: [Typ; TYP_MATRIX_BIT_SHIFT_LEN] = initialize_typ_matrix_bit_shift();
//lets go memory brute force
//no init time, due to const fn!! calculate at compile time
//this can actually be much smaller,
//as our range reduction produces sorted outputs:
//max access value = 0, 1, 2, 3, 4
//-> 1*2*3*4*5 = 120 instead of 3125
pub const fn initialize_typ_matrix_bit_shift() -> [Typ; TYP_MATRIX_BIT_SHIFT_LEN] {
    let mut typ_matrix = [Typ::HighCard; TYP_MATRIX_BIT_SHIFT_LEN];
    
    /*
    When accessing, i0 is always 0.
0b333_22_11_0
     */
    let mut i: usize = 0;
    //highcard number is always 0b100_11_10_1,
    //nothing bigger possible.
    //could sip all 0b000_00_11_0 options though
    while i < TYP_MATRIX_BIT_SHIFT_LEN { /* 151, the max number */ 
        //this is basically the same approach as just calculating the hand on the fly xD
        let typ = identify_hand_type(i);
        typ_matrix[i] = typ;                        
        i += 1;
    }

    typ_matrix
}

pub const fn identify_hand_type(cards: usize) -> Typ {
    //card to occurrence count mapping
    let mut occurrences = [0 as usize; 5];
    //card1 occurences
    occurrences[0] = 1; // 1. card (is always 0 and therefor alwats there)
    let card = cards & 0b000_00_00_1; //0 - 1
    occurrences[card] += 1; // 2. card
    let card = (cards & 0b000_00_11_0) >> 1; //0 - 2
    occurrences[card] += 1; // 3. card
    let card = (cards & 0b000_11_00_0) >> 3; //0 - 3
    occurrences[card] += 1; // 4. card
    let card = (cards & 0b111_00_00_0) >> 5; //0 - 4
    occurrences[card] += 1; // 5. card

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

/// Bench Results:
/// due to range reduction the first index is always 0, therefor omitting it.
/// the max returned value is in range [0,0,0,0] to [1,2,3,4]
///             => is just a thought about 7bit, 120 possibilities
/// [0,0,0,0] => 0
/// [0,0,0,1] => 5
/// [0,0,1,1] => 4+5
/// [0,1,1,1] => 3+4+5
/// [1,1,1,1] => 2+3+4+5
/// [1,1,1,0] => 2+3+4
/// [1,1,0,0] => 2+3
/// [1,0,0,0] => 2
/// [1,0,0,1] => 2+5
/// 
/// [0,0,1,2] => 4+10
/// [0,1,2,2]
/// [1,2,2,2]
/// [1,2,2,3]
/// [1,2,3,3]
/// [1,2,3,4] => 2+6+12+20
///
/// nr of options and bits required to address index:
/// 0: 2= 1
/// 1: 3= 2
/// 2: 4= 2
/// 3: 5= 3
/// 
/// addressing:
/// 01122333
/// 
/// so we could theoretically return a tuple of [1bit, 2bit, 2bit, 3bit]
/// -> encode the result in 1 byte
/// actually, 2*3*4*5 = 120, e.g. 7bit would suffice, but not with simple bit shifing..
/// 
/// address pattern:
/// 0b0_11_22_333 = 8bit //no, 0b333_22_11_0!
/// 
/// 
/// return 0b01223_3444
pub fn reduce_variant_range_bit_shift(cards: [usize; 5]) -> usize {
    //init with 8, which is > the max real value of 4
    let mut variant_map = [8 as usize; 13];
    let mut result: usize = 0;
    //first is always 0, result[0] therefor also 0, and therefor can be omitted
    variant_map[cards[0]] = 0;

    //as card[0] is always mapped to 0, we can just omitt one dimension in the result

    // address pattern:
    // 0b0_11_22_333 = 8bit
    // 0b333_22_11_0 = 8bit !! this one


    //card 1 can be 0 or 1
    result = result | match variant_map[cards[1]] {
        8 => {
            variant_map[cards[1]] = 1;
            0b000_00_00_1
        },
        _ => 0 //only 0 is the other option //variant_map[cards[1]]
    };
    result = result | match variant_map[cards[2]] {
        8 => {
            variant_map[cards[2]] = 2;
            0b000_00_10_0
        },
        _ => variant_map[cards[2]] << 1 // 0 or 1, shift to the left 1, to skip the card[1] bytes
    };
    result = result | match variant_map[cards[3]] {
        8 => {
            variant_map[cards[3]] = 3;
            0b000_11_00_0
        },
        _ => variant_map[cards[3]] << 3 // skip the card[2] position, shift to pos of the 0b11 in 0b000_11_00_0
    };
    result = result | match variant_map[cards[4]] {
        8 => {
            variant_map[cards[4]] = 4;
            0b100_00_00_0 //TODO: will do with 8 bit for now, but I think it could be reduced to at least 7 bit
        },
        _ => variant_map[cards[4]] << 5
    };

    //can reduce the8 bit to 7 bit:
    //e.g. 128 possibilities:
    // 33_22_11_0
    // but as 11 is never 3, just (0-2), => (card & 0b00_00_11_0) == 0b00_00_11_0  could be 4!
    // !! but this hides the 0b01_0, 0b10_0 or 0b00_0 that was in card 2 position :(

    // // 0b333_22_11_0 = 8bit
    // return card0 & card1 << 1 & card2 << 3 & card3 << 5;// as u8;
    // 0b33_22_11_0 = !7bit
    return result;
}

#[cfg(test)]
mod tests {
    use crate::y2023::day7_1_7bit_matrix::{*};

    #[test]
    fn test_reduce_variant_range_bit_shift() {
        // input == output
        assert_eq!(0b000_00_00_0, reduce_variant_range_bit_shift([0, 0, 0, 0, 0]));
        assert_eq!(0b100_00_00_0, reduce_variant_range_bit_shift([0, 0, 0, 0, 1]));
        assert_eq!(0b011_11_00_0, reduce_variant_range_bit_shift([0, 0, 0, 1, 1]));
        assert_eq!(0b100_11_00_0, reduce_variant_range_bit_shift([0, 0, 0, 1, 2]));
        assert_eq!(0b100_10_10_0, reduce_variant_range_bit_shift([0, 0, 1, 1, 2]));
        assert_eq!(0b100_11_10_0, reduce_variant_range_bit_shift([0, 0, 1, 2, 3]));
        assert_eq!(0b100_11_10_1, reduce_variant_range_bit_shift([0, 1, 2, 3, 4]));

        //with actual different ranges
        //this test shows that we dimensioned the TYP_MATRIX_BIT_SHIFT map way to big!
        //skip the first dimension, second has size 2 etc..
        assert_eq!(0b000_00_00_0, reduce_variant_range_bit_shift([12, 12, 12, 12, 12]));
        assert_eq!(0b000_00_10_0, reduce_variant_range_bit_shift([3, 3, 1, 3, 3]));
        assert_eq!(0b000_01_01_1, reduce_variant_range_bit_shift([7, 0, 0, 0, 7]));
        assert_eq!(0b010_10_10_1, reduce_variant_range_bit_shift([3, 0, 2, 2, 2]));
        assert_eq!(0b100_10_10_0, reduce_variant_range_bit_shift([0, 0, 11, 11, 12]));
        assert_eq!(0b100_11_10_0, reduce_variant_range_bit_shift([0, 0, 7, 8, 4]));
        assert_eq!(0b100_11_10_1, reduce_variant_range_bit_shift([8, 9, 10, 11, 12]));
    }

    #[test]
    fn test_initialize_typ_matrix_bit_shift() {
        let typ_matrix = initialize_typ_matrix_bit_shift();

        
        let idx = reduce_variant_range_bit_shift([0, 0, 0, 0, 0]);
        assert_eq!(Typ::FiveOfAKind, typ_matrix[idx]);
        let idx = reduce_variant_range_bit_shift([0, 0, 0, 0, 1]);
        assert_eq!(Typ::FourOfAKind, typ_matrix[idx]);
        let idx = reduce_variant_range_bit_shift([0, 0, 0, 1, 1]);
        assert_eq!(Typ::FullHouse, typ_matrix[idx]);
        let idx = reduce_variant_range_bit_shift([0, 0, 0, 1, 2]);
        assert_eq!(Typ::ThreeOfAKind, typ_matrix[idx]);
        let idx = reduce_variant_range_bit_shift([0, 0, 1, 1, 2]);
        assert_eq!(Typ::TwoPair, typ_matrix[idx]);
        let idx = reduce_variant_range_bit_shift([0, 0, 1, 2, 3]);
        assert_eq!(Typ::OnePair, typ_matrix[idx]);
        let idx = reduce_variant_range_bit_shift([0, 1, 2, 3, 4]);
        assert_eq!(Typ::HighCard, typ_matrix[idx]);
    }

    #[test]
    fn test_identify_hand_type() {
        //test with max 5 variants
        assert_eq!(Typ::FiveOfAKind, identify_hand_type(0b000_00_00_0));
        assert_eq!(Typ::FourOfAKind, identify_hand_type(0b001_00_00_0));
        assert_eq!(Typ::FullHouse, identify_hand_type(0b001_01_00_0));
        assert_eq!(Typ::ThreeOfAKind, identify_hand_type(0b010_01_00_0));
        assert_eq!(Typ::TwoPair, identify_hand_type(0b010_01_01_0));
        assert_eq!(Typ::OnePair, identify_hand_type(0b011_10_01_0));
        assert_eq!(Typ::HighCard, identify_hand_type(0b100_11_10_1));
    }

    #[test]
    fn test_identify_hand_type_illegal_combinations() {
        //test with nothing where card 4 is gt 5!

        //test with max 5 variants
        // assert_eq!(Typ::FiveOfAKind, identify_hand_type(0b111_00_00_0)); //only 5 options, not 8
        assert_eq!(Typ::FourOfAKind, identify_hand_type(0b000_00_11_0)); //works, even though card 2 has the illegal value 3, as 5 is the limit
        assert_eq!(Typ::OnePair, identify_hand_type(0b100_11_11_1)); //just all ones, lol, except card5, we have a limit of max 5
        
        //is actually the highest number, e.g. just iterate until that = 151
        //and skip the range 0b000_00_11_0 completely
        assert_eq!(Typ::HighCard, identify_hand_type(0b100_11_10_1)); 
    }
}