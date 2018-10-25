#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::num::IntSqrt;

#[bench]
fn isqrt_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let sum = (1u64..1_000_000)
            .map(|n| n.isqrt())
            .sum::<u64>();
        test::black_box(sum);
    })
}

#[bench]
fn is_square_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let sum = (1u64..1_000_000)
            .filter(|n| n.is_square())
            .sum::<u64>();
        test::black_box(sum);
    })
}

#[bench]
fn sqrt_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let sum = (1u64..1_000_000)
            .filter_map(|n| n.sqrt())
            .sum::<u64>();
        test::black_box(sum);
    })
}
