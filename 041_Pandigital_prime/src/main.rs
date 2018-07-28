#![feature(test)]
extern crate test;
extern crate prime;

fn find_biggest_pandigital_prime ( prime_numbers:&[u64] ) -> Option<u64> {
	'primes : for &prime in prime_numbers.iter().rev() {
		let mut digits_found = [false;9];
		let prime_string : String = prime.to_string();
		for ch in prime_string.chars() {
			let digit:u32 = ch.to_digit(10).unwrap();
			if digit == 0 { continue 'primes };
			if !digits_found[(digit-1) as usize] {
				digits_found[(digit-1) as usize] = true;
			}
			else { continue 'primes; }
		}

		for (n,&digit_present) in digits_found.iter().enumerate() {
			if !digit_present { continue 'primes }; // failed
			if n+1 == prime_string.len() { break }; // passed
		}
		return Some(prime);
	}
	None
}

fn main() {
	let max_prime = 7654321; // every pandigital above divisible by 3
	let prime_numbers = prime::sieve(max_prime);

	if let Some(biggest_pandigital_prime) = find_biggest_pandigital_prime(&prime_numbers) {
		println!("{}", biggest_pandigital_prime);
	}
}

#[bench]
fn find_biggest_pandigital_prime_from_list (b:&mut test::Bencher) {
	let primes = prime::sieve(7654321);
	b.iter(|| find_biggest_pandigital_prime(&primes));
}

#[bench]
fn find_primes (b:&mut test::Bencher) {
	b.iter(|| prime::sieve(7654321));
}
