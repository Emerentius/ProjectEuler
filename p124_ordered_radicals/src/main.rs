#![feature(test)]
#![feature(slice_patterns)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime;

fn main() {
    let limit = 100_000;
    let primes = prime::sieve( (limit as f64).sqrt() as usize + 1 );
    let mut rads = vec![(1,1)];
    for n in 2..limit+1 {
        let factors = prime::factors(n as u64, &primes);
        let rad = factors.into_iter().fold(1, |prod, [pr, _]| prod*pr);
        rads.push((rad, n));
    }
    rads.sort();
    println!("{:?}", rads[10_000 - 1].1);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
