extern crate euler_utils;
use euler_utils::prime;

fn main() {
	let primes = prime::sieve(1_000_000);
	
	let mut max_seq_len = 0;
	let mut max_base_prime = 0;
	
	for (i,&prime) in primes.iter().enumerate() {
		let mut sum = 0;
		
		let iter1 = (&primes[..i]).iter().enumerate();
		let mut iter2 = (&primes[..i]).iter();
		let mut low = 0;
		for (upp,&prime_upp) in iter1 {
			sum += prime_upp;
			while sum > prime {
				if let Some(&prime_low) = iter2.next() {
					sum -= prime_low;
					low += 1;
				}
			}
			let seq_len = upp as u64 - low + 1;
			if sum == prime && seq_len > max_seq_len {
				max_seq_len = seq_len;
				max_base_prime = prime;
			}
		}
	}
	println!("prime: {}, sequence length: {}", max_base_prime, max_seq_len);
}
