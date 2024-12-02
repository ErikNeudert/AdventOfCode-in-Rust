use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
// qualify directly for better readability
use rust_aoc::y2023::day7_1;
use rust_aoc::y2023::day7_1_slow_methods;
use rust_aoc::y2023::day7_1_7bit_matrix;
use rust_aoc::y2023::day7_1_matrix_13;

pub fn bench_initialize_typ_matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("initialize_typ_matrix");

    group.bench_function("5^5 array", |b| b.iter(|| black_box(day7_1::initialize_typ_matrix())));
    group.bench_function("5^13 array", |b| b.iter(|| black_box(day7_1::initialize_typ_matrix())));
    group.bench_function("bit_shift", |b| b.iter(|| black_box(day7_1_7bit_matrix::initialize_typ_matrix_bit_shift())));

    group.finish();
}

/// tests the transformation of [usize;5] card array to Typ
pub fn bench_identify_hand_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("identify_hand_type");

    let inputs: [[usize; 5]; 2] = [
        [0, 1, 2, 3, 4],
        [0, 3, 9, 5, 12]
    ];
    for (idx, input) in inputs.into_iter().enumerate() {
        // group.bench_with_input(BenchmarkId::new("identify_hand_type reduce_variant_range_slow", idx), 
        //     &input, |b, input| b.iter(|| {
        //         let cards_reduced_range = day7_1_slow_methods::reduce_variant_range_slow(*input);
        //         let typ = day7_1::identify_hand_type(cards_reduced_range);
        //         black_box(typ);
        //     })
        // );
        // group.bench_with_input(BenchmarkId::new("identify_hand_type reduce_variant_range", idx), 
        //     &input, |b, input| b.iter(|| {
        //         let cards_reduced_range = day7_1::reduce_variant_range(*input);
        //         let typ = day7_1::identify_hand_type(cards_reduced_range);
        //         black_box(typ);
        //     })
        // );
        // group.bench_with_input(BenchmarkId::new("TYP_MATRIX reduce_variant_range_slow", idx), 
        //     &input, |b, input| b.iter(|| {
        //         let cards_reduced_range = day7_1_slow_methods::reduce_variant_range_slow(*input);
        //         let typ = day7_1::TYP_MATRIX[cards_reduced_range[0]]
        //             [cards_reduced_range[1]]
        //             [cards_reduced_range[2]]
        //             [cards_reduced_range[3]]
        //             [cards_reduced_range[4]];
        //         black_box(typ);
        //     })
        // );
        group.bench_with_input(BenchmarkId::new("TYP_MATRIX (bit_shift)", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = day7_1_7bit_matrix::reduce_variant_range_bit_shift(*input);
                let typ = day7_1_7bit_matrix::TYP_MATRIX_BIT_SHIFT[cards_reduced_range];
                black_box(typ);
            })
        );
        group.bench_with_input(BenchmarkId::new("TYP_MATRIX (static)", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = day7_1::reduce_variant_range(*input);
                let typ = day7_1::TYP_MATRIX[cards_reduced_range[0]]
                    [cards_reduced_range[1]]
                    [cards_reduced_range[2]]
                    [cards_reduced_range[3]]
                    [cards_reduced_range[4]];
                black_box(typ);
            })
        );
        group.bench_with_input(BenchmarkId::new("TYP_MATRIX 13^5", idx), 
            &input, |b, input| b.iter(|| {
                let typ = day7_1::TYP_MATRIX[input[0]]
                    [input[1]]
                    [input[2]]
                    [input[3]]
                    [input[4]];
                black_box(typ);
            })
        );
        group.bench_with_input(BenchmarkId::new("identify_hand_type13", idx), 
            &input, |b, input| b.iter(|| {
                let typ = day7_1_slow_methods::identify_hand_type13(*input);
                black_box(typ);
            })
        );
        group.bench_with_input(BenchmarkId::new("identify_hand_type13", idx), 
            &input, |b, input| b.iter(|| {
                let typ = day7_1_slow_methods::identify_hand_type13(*input);
                black_box(typ);
            })
        );
    }

    group.finish();
}

/*
git commit -m "Day 7.1: 13^5 type matrix for direct card access

- i incorrectly assumed i would need 13^13 for direct access via card num

Benchmarking Results:
static variant reduction + type matrix (len 5^5):
bit_shift + matrix with len 152:
type matrix 13 (len 13^5):
"
*/

/*
git commit -m "Day 7.1: 8bit Matrix access Benchmarks and Impl

- implement a bit shifting algorithm for an array of length 152 instead of 5^5
- added all 7_1 to lib.rs for testing and bench usability
- warning cleanup in module 7_1_slow_methods
- added an input for each hand type to bench_reduce_variant_range
- removed slow benchmarking methods to improve benching time
- removed unnecessary variable identify hand type method

Benchmarking Results:
    reduce_variant_range
        - bit shift: ~ 5.0746 ns
        - static: ~ 5.9092 ns
    identify_hand_type
        type_matrix
            - bit shift:
            - static:
            - half static:
            - dynamic 13: 
static variant reduction + type matrix (len 5^5): ~
bit_shift + matrix with len 152:


"
*/
pub fn bench_reduce_variant_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("reduce_variant_range");
// 2024/04/28 10:12: theoreticallty unstabler benches due to low power mode
    let inputs: [[usize; 5]; 7] = [
        //using the input values that require theoretically more range transformation
        [12, 6, 3, 2, 1], //HighCard
        [2, 6, 3, 12, 2],//Pair
        [12, 6, 3, 12, 3],//TwoPair
        [12, 6, 3, 3, 3],//ThreeOfAKind
        [12, 6, 6, 6, 12],//FullHouse
        [11, 12, 12, 12, 12],//FourOfAKind
        [12, 12, 12, 12, 12],//FiveOfAKind
    ];
    for (idx, input) in inputs.into_iter().enumerate() {
        // group.bench_with_input(BenchmarkId::new("half static", idx), 
        //     &input, |b, input| b.iter(|| {
        //         let cards_reduced_range = day7_1_slow_methods::reduce_variant_range_half_static(*input);
        //         black_box(cards_reduced_range);
        //     })
        // );
        group.bench_with_input(BenchmarkId::new("bit shift", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = day7_1_7bit_matrix::reduce_variant_range_bit_shift(*input);
                black_box(cards_reduced_range);
            })
        );
        group.bench_with_input(BenchmarkId::new("static", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = day7_1::reduce_variant_range(*input); //full static
                black_box(cards_reduced_range);
            })
        );
        // group.bench_with_input(BenchmarkId::new("reduce_variant_range_slow", idx), 
        //     &input, |b, input| b.iter(|| {
        //         let cards_reduced_range = day7_1_slow_methods::reduce_variant_range_slow(*input);
        //         black_box(cards_reduced_range);
        //     })
        // );
    }

    group.finish();
}

criterion_group!(benches, bench_initialize_typ_matrix, bench_identify_hand_type);
criterion_main!(benches);
