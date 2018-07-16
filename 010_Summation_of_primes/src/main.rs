#![feature(test)]
extern crate test;
extern crate prime;

fn main() {
	let mut primes = prime::sieve(2_000_000);
	let mut prime_sum : u64 = primes.into_iter().fold(0,|sum, p| sum + p);
	println!("{}", prime_sum);
}

#[bench]
fn bench(b:&mut test::Bencher) {
	b.iter(|| main())
}
