#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime;

fn main() {
	let primes = prime::sieve(2_000_000);
	let prime_sum : u64 = primes.into_iter().fold(0,|sum, p| sum + p);
	println!("{}", prime_sum);
}

#[bench]
fn bench(b:&mut test::Bencher) {
	b.iter(|| main())
}
