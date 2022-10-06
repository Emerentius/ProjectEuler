#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime; // own library

fn main() {
    let primes = prime::sieve(100); // will escalate fast, think factorial
    let mut number = 1;
    for &prime in &primes {
        number *= prime;
        if number > 1_000_000 {
            number /= prime;
            break;
        }
    }
    println!("{}", number);
}

#[bench]
fn bench (b:&mut test::Bencher) {
    b.iter(|| main())
}
