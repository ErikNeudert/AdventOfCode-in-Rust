use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_aoc::day7_1::*;

pub fn bench_initialize_typ_matrix(c: &mut Criterion) {
    c.bench_function("initialize_typ_matrix", |b| b.iter(|| black_box(initialize_typ_matrix())));
}

pub fn bench_identify_hand_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("identify_hand_type");

    let inputs: [[usize; 5]; 2] = [
        [0, 1, 2, 3, 4],
        [0, 3, 9, 5, 12]
    ];
    for (idx, input) in inputs.into_iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("reduce_variant_range", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = reduce_variant_range(*input);
                black_box(cards_reduced_range);
            })
        );
        group.bench_with_input(BenchmarkId::new("reduce_variant_range_static", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = reduce_variant_range_static(*input);
                black_box(cards_reduced_range);
            })
        );
        group.bench_with_input(BenchmarkId::new("dynamic lookup HighCard identify_hand_type", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = reduce_variant_range(*input);
                let typ = identify_hand_type(cards_reduced_range);
                black_box(typ);
            })
        );
        group.bench_with_input(BenchmarkId::new("dynamic lookup HighCard reduce_variant_range_static", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = reduce_variant_range_static(*input);
                let typ = identify_hand_type(cards_reduced_range);
                black_box(typ);
            })
        );
        group.bench_with_input(BenchmarkId::new("dynamic lookup HighCard identify_hand_type13", idx), 
            &input, |b, input| b.iter(|| {
                let typ = identify_hand_type13(*input);
                black_box(typ);
            })
        );
        group.bench_with_input(BenchmarkId::new("static lookup HighCard TYP_MATRIX reduce_variant_range", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = reduce_variant_range(*input);
                let typ = TYP_MATRIX[cards_reduced_range[0]]
                    [cards_reduced_range[1]]
                    [cards_reduced_range[2]]
                    [cards_reduced_range[3]]
                    [cards_reduced_range[4]];
                black_box(typ);
            })
        );
        group.bench_with_input(BenchmarkId::new("static lookup HighCard TYP_MATRIX reduce_variant_range_static", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = reduce_variant_range_static(*input);
                let typ = TYP_MATRIX[cards_reduced_range[0]]
                    [cards_reduced_range[1]]
                    [cards_reduced_range[2]]
                    [cards_reduced_range[3]]
                    [cards_reduced_range[4]];
                black_box(typ);
            })
        );
    }

    group.finish();
}

pub fn bench_reduce_variant_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("reduce_variant_range");

    let inputs: [[usize; 5]; 2] = [
        [0, 1, 2, 3, 4],
        [0, 3, 9, 5, 12]
    ];
    for (idx, input) in inputs.into_iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("loop", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = reduce_variant_range(*input);
                black_box(cards_reduced_range);
            })
        );
        group.bench_with_input(BenchmarkId::new("static", idx), 
            &input, |b, input| b.iter(|| {
                let cards_reduced_range = reduce_variant_range_static(*input);
                black_box(cards_reduced_range);
            })
        );
    }

    group.finish();
}


criterion_group!(benches, bench_initialize_typ_matrix, bench_identify_hand_type, bench_reduce_variant_range);
criterion_main!(benches);
