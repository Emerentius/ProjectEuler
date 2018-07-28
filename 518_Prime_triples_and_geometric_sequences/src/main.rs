extern crate primal;
extern crate num;
use num::integer::gcd;

const LIMIT: usize = 100_000_000;

// x, y, z = a + 1, b + 1, c + 1
// geometric sequence
// z = r*y, y = r*x, r = p/q where gcd(p,q) = 1
// z = y * p/q = x * p²/q²
//
// => x | q²
// x = k * q²
// y = k * q² * p/q = k * pq
// z = k * p²
//
// x < y < z < n+1
// => p > q
//    p²/q² < p² < n+1
//
// iterate over all k, and p/q such that c < n
// check a, b, c for primality
fn main() {
	let sieve = primal::Sieve::new(LIMIT);

	let mut sum = 0;
	// iterate over all combinations of k, p, q in the order
	// that allows to skip as much as possible
	for k in 2..LIMIT/4 {
		for p in 2.. {
			let c = k*p*p - 1;
			if c >= LIMIT {	break }
			if !sieve.is_prime(c) { continue }
			for q in 1..p {
				let a = k*q*q - 1;
				let b = k*q*p - 1;
				if sieve.is_prime(a) && sieve.is_prime(b) && gcd(p, q) == 1 {
					sum += a + b + c;
				}
			}
		}
	}

	println!("{}", sum);
}
