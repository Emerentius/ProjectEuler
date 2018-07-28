#![feature(test)]
extern crate test;
extern crate num;
use num::bigint::ToBigInt;
use num::BigInt;

fn main() {
	let mut numbers : Vec<BigInt> = vec![];

	for a in 2..101 {
		for b in 2..101 {
			let a_big = a.to_bigint().unwrap();
			numbers.push(num::pow(a_big, b));
		}
	}

	numbers.sort();
	numbers.dedup();
	println!("{}", numbers.len() );
}

#[cfg(test)]
#[bench]
fn bench(b: &mut test::Bencher) { b.iter(|| main()) }
