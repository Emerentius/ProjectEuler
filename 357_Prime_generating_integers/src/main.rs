#![feature(test)]
extern crate test;
extern crate primal;
extern crate bitvec;

const LIMIT: usize = 100_000_000;

fn main (){
	let sieve = primal::Sieve::new(LIMIT);
	let candidates = sieve.primes_from(1)
		.take_while(|&pr| pr < LIMIT)
		.map(|p| p-1);

	let mut sum = 0;
	'cand: for cand in candidates {
		let upper_limit = (cand as f64).sqrt() as usize;

		for n in (2..upper_limit+1).filter(|&n| cand % n == 0) {
			if !sieve.is_prime(n+cand/n) { continue 'cand }
		}
		sum += cand;
	}
	println!("{}", sum);
}

#[bench]
fn bench (b: &mut test::Bencher) {
	b.iter(|| main());
}
