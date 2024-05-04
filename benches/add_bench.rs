// #![feature(proc_macro_c_str_literals)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ruint::aliases::U256;
use ruint_simd::U256x4;

fn simd_add(lhs: U256x4, rhs: U256x4) -> U256x4 {
    lhs + rhs
}

fn arr_add(lhs: [U256; 4], rhs: [U256; 4]) -> [U256; 4] {
    [
        lhs[0] + rhs[0],
        lhs[1] + rhs[1],
        lhs[2] + rhs[2],
        lhs[3] + rhs[3],
    ]
}

fn simd_add_bench(c: &mut Criterion) {
    let a = U256x4::new(
        U256::from_limbs([1, 2, 3, 4]),
        U256::from_limbs([5, 6, 7, 8]),
        U256::from_limbs([9, 10, 11, 12]),
        U256::from_limbs([13, 14, 15, 16]),
    );

    let b = U256x4::new(
        U256::from_limbs([17, 18, 19, 20]),
        U256::from_limbs([21, 22, 23, 24]),
        U256::from_limbs([25, 26, 27, 28]),
        U256::from_limbs([29, 30, 31, 32]),
    );

    c.bench_function("simd", |benches| {
        benches.iter(|| simd_add(black_box(a), black_box(b)))
    });
}

fn arr_add_bench(c: &mut Criterion) {
    let a = [
        U256::from_limbs([1, 2, 3, 4]),
        U256::from_limbs([5, 6, 7, 8]),
        U256::from_limbs([9, 10, 11, 12]),
        U256::from_limbs([13, 14, 15, 16]),
    ];

    let b = [
        U256::from_limbs([17, 18, 19, 20]),
        U256::from_limbs([21, 22, 23, 24]),
        U256::from_limbs([25, 26, 27, 28]),
        U256::from_limbs([29, 30, 31, 32]),
    ];

    c.bench_function("array", |benches| {
        benches.iter(|| arr_add(black_box(a), black_box(b)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = simd_add_bench, arr_add_bench
}
criterion_main!(benches);
