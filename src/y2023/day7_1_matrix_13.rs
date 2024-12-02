// use crate::day7_1::{Hand, Typ};
use crate::y2023::day7_1::Typ;
use crate::y2023::day7_1_slow_methods;

// Memory brute force: 
// nr accesses: 1
// memory: 5dim array, 13^5=371.293 * (5 cards + 1 Typ)
//     all possibilities in a matrix of theoretical size:
//         3bit for 7 Typ possibilities
//         371.293 matrix points * 3bit = 1.113.879 = 135kb 
// (i incorrectly calculated the 5^5 matrix in day7_1.rs)
pub const MATRIX_LEN: usize = 13;
#[warn(long_running_const_eval)]
pub const TYP_MATRIX: [[[[[Typ; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN] = initialize_typ_matrix_13();
//lets go memory brute force
//no init time, due to const fn!! calculate at compile time
pub const fn initialize_typ_matrix_13() -> [[[[[Typ; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN] {
    let mut typ_matrix = [[[[[Typ::HighCard; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN]; MATRIX_LEN];
    
    let mut i0: usize = 0;
    while i0 < MATRIX_LEN {
        let mut i1: usize = 0;
        while i1 < MATRIX_LEN {
            let mut i2: usize = 0;
            while i2 < MATRIX_LEN {
                let mut i3: usize = 0;
                while i3 < MATRIX_LEN {
                    let mut i4: usize = 0;
                    while i4 < MATRIX_LEN {
                        //this is basically the same approach as just calculating the hand on the fly xD
                        let typ = day7_1_slow_methods::identify_hand_type13([i0, i1, i2, i3, i4]);
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

#[cfg(test)]
mod tests {
    use crate::y2023::day7_1_matrix_13::{*};

    #[test]
    fn test_initialize_typ_matrix_13() {
        let typ_matrix = initialize_typ_matrix_13();
        
        assert_eq!(Typ::FiveOfAKind, typ_matrix[0][0][0][0][0]);
        assert_eq!(Typ::FourOfAKind, typ_matrix[0][0][0][0][1]);
        assert_eq!(Typ::FullHouse, typ_matrix[0][0][0][1][1]);
        assert_eq!(Typ::ThreeOfAKind, typ_matrix[0][0][0][1][2]);
        assert_eq!(Typ::TwoPair, typ_matrix[0][0][1][1][2]);
        assert_eq!(Typ::OnePair, typ_matrix[0][0][1][2][3]);
        assert_eq!(Typ::HighCard, typ_matrix[0][1][2][3][4]);
    }
}