#![feature(test)]
extern crate test;

extern crate primal; // a rust prime library
use primal::is_prime; // deterministic Miller-Rabin

fn main() {
    print!("{}",
        (1u64..).map(|i| (i+1).pow(3) - i.pow(3))
            .take_while(|&n| n < 1_000_000)
            .filter(|&num| is_prime(num))
            .count()
    );
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
