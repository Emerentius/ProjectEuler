#![feature(test)]
extern crate test;
extern crate prime;

fn main() {
	let modulo = 500500507;
	let goal_exp = 500500;
	let max_prime = 7376507; // this is the 500500th prime (found out by prior calculation)
	let primes = prime::sieve(max_prime);

	let mut lowest_composite = 1;
	// Vec<[prime, occurences]>, prime factors of lowest_composite
	let mut composited_primes : Vec<[u64; 2]> = vec![];
	let mut divisors_exp = 0; // 2^divisors_exp divisors

	'new_prime:for (i,&new_prime) in primes.iter().enumerate() {
		let new_prime = new_prime as u64;
		// check for higher prime powers later on only
		// there won't be many increases
		if goal_exp - i < 1000 {
			let mut old_occ = 0;
			'old_prime:for &mut[old_prime, ref mut occ] in composited_primes.iter_mut() {
				// does old prime yield a lower composite number when doubling divisors?
				if old_occ == *occ {
					if *occ == 1 { break 'old_prime };
					old_occ = *occ;
					continue 'old_prime
				};
				old_occ = *occ;

				let multiplicand = old_prime.pow( (*occ+1) as u32 );
				if multiplicand < new_prime {
					lowest_composite = lowest_composite * multiplicand % modulo;
					*occ = (*occ+1) * 2 - 1;
					divisors_exp += 1;
					if divisors_exp == goal_exp { break 'new_prime };
				}
			}
		}
		lowest_composite = new_prime * lowest_composite % modulo;
		composited_primes.push([new_prime, 1]);
		divisors_exp += 1;
		if divisors_exp == goal_exp { break 'new_prime};
	}
	println!("Lowest composite number with 2^500500 divisors is: {}", lowest_composite);
}

#[bench]
fn bench ( b:&mut test::Bencher) {
	b.iter(|| main() );
}

#[bench]
fn primes ( b:&mut test::Bencher) {
	b.iter(|| prime::sieve(7376507));
}
