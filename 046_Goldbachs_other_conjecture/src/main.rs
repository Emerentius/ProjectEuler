extern crate test;
extern crate prime;
use prime::is_prime;

fn main() {
	let primes = prime::sieve(1_000_000);
	'n :for n in std::iter::count(1,2) {
		if is_prime(n, &primes) { continue }
		'prime: for &prime in primes.iter().skip(1) {
			if prime > n {
				println!("{}", n);
				break 'n;
			}
			let mut diff = (n - prime)/2;
			'sq: for m in 1u64.. {
				let two_sq = m*m;
				if diff == two_sq { break 'prime }
				if diff < two_sq { continue 'prime }
			}
		}
	}
}

#[bench]
fn bench( b: &mut test::Bencher) {
	b.iter( || main() );
}