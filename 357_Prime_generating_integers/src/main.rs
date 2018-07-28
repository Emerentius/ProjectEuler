#![feature(collections)]
#![feature(step_by)]
#![feature(test)]
extern crate test;
extern crate prime;
use std::collections::HashSet;
use std::collections::BitVec;

fn eras_sieve ( max_prime : usize ) -> Vec<bool> {
	let mut is_prime = vec![true; max_prime+1];
	is_prime[0]=false;
	is_prime[1]=false;

	for number in 0..is_prime.len() {
		if is_prime[number] {
			for i in (number*number..max_prime+1).step_by(number) {
				is_prime[i] = false;
			}
		}
	}
	is_prime
}


fn main (){
	let is_prime = eras_sieve(100_000_002);
	let primes = prime::sieve(100_000_000);
	let candidates = primes.into_iter().map(|p| p-1);

	let mut sum = 0;
	'cand: for cand in candidates {
		let upper_limit = (cand as f64).sqrt() as u64;
		let mut divisor = cand;

		for n in (2..upper_limit+1).filter(|&n| cand % n == 0) {
			if !is_prime[ (n+cand/n) as usize ] { continue 'cand }
		}
		sum += cand;
	}
	println!("{}", sum);
}

#[bench]
fn bench (b: &mut test::Bencher) {
	b.iter(|| main());
}
