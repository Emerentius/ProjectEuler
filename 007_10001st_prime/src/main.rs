// prime calculation copied from 004
use std::num::Float;

fn main() {
	let mut prime_numbers = vec::<i64>![2];
	let nth_prime = 10001; // find n-th prime
	prime_numbers.reserve(nth_prime);
	let mut not_a_prime = false;
	for x in 3.. {
		let x_root = 1 + (x as f64).sqrt() as i64; // +1 in case of rounding errors
		for y in prime_numbers.iter() {
			if x % *y == 0 {
				not_a_prime = true; 
				break;
			}
			if x_root < *y {break;};
		}
		if !not_a_prime { 
			prime_numbers.push(x);
			//if prime_numbers.len() % 100 == 0 { println!("{}", prime_numbers.len())}
		}
		not_a_prime = false;
		
		if prime_numbers.len() == nth_prime { break; }
	};
	println!("{}", prime_numbers[nth_prime-1]);
}