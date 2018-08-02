#![feature(test)]
extern crate test;
extern crate prime;
use std::collections::HashSet;

fn main() {
	let primes = prime::sieve(1_000_000); // just a guess
	let mut primes_set = HashSet::new();
	primes_set.reserve( primes.len() );
	// enter primes that will be skipped in main loop
	for &prime in [2,3,5,7].iter() { primes_set.insert( prime ); }

	let mut sum = 0;
	let mut truncatable_primes = vec![];
	'prime: for &prime in primes.iter().skip(4) {
		primes_set.insert( prime );
		let prime_str = prime.to_string();
		let length = prime_str.len();
		for cut in (1..length).rev() {
			let trunc_right : u64 = (&prime_str[..length-cut]).parse().unwrap();
			let trunc_left  : u64 = (&prime_str[cut..]).parse().unwrap();

			if !primes_set.contains(&trunc_left) || !primes_set.contains(&trunc_right) {
				continue 'prime; // not truncatable
			}
		}
		// passed truncability test
		truncatable_primes.push(prime);
		sum += prime;
		if truncatable_primes.len() == 11 { break };
	}

	println!("{}", sum);
	//println!("{:?}", truncatable_primes);
}

#[bench]
fn bench (b: &mut test::Bencher) {
	b.iter(|| main() );
}
