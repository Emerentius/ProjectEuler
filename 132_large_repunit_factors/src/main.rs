#![feature(test)]
extern crate test;

extern crate num;
extern crate primal;

use num::integer::gcd;

const K: u64 = 1_000_000_000;

fn pow_mod(mut number:u64, mut power:u64, modu:u64) -> u64 {
	let mut result = 1;
	while power != 0 {
		if power % 2 != 0 { result = (result*number) % modu }
		number = (number*number) % modu;
		power /= 2;
	}
	result
}

fn main() {
    let solution: u64 = primal::Primes::all()
		.map(|pr| pr as u64)
		// .filter(|&pr| pow_mod(10, K, 9*pr) == 1) // works just as well, but a bit slower
		.filter(|&pr| pow_mod(10, gcd(K, pr - 1), 9*pr) == 1)
        .take(40)
        .sum();
    println!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
