#![feature(test)]
extern crate test;

extern crate prime; // my own library, erasthothenes sieve and trial division

fn is_circular_prime(prime:u64) -> bool {
	let mut digits : Vec<u64> = vec![];
	for ch in prime.to_string().chars().rev() {
		digits.push( ch.to_digit(10).unwrap() as _ )
	}

	for rotation in digits.iter().cycle().take(2*digits.len() - 1).collect::<Vec<&u64>>().windows( digits.len() ) {
		let mut num = 0;
		for (i,&digit) in rotation.iter().enumerate() {
			if digit % 2 == 0 || digit % 5 == 0 { return false } // slight optimisation
			num += digit * 10u64.pow(i as u32);
		}

		if !prime::is_prime(num) { return false }
	}
	true
}

fn main() {
	let primes = prime::sieve(1_000_000);

	let mut counter = 2; // 2 and 5 slips through due to small optimisation
	for prime in primes {
		if is_circular_prime(prime) { counter += 1 }
	}
	println!("{}", counter);
}

#[bench]
fn bench (b:&mut test::Bencher) {
	b.iter(|| main() );
}
