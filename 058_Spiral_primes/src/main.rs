#![feature(test)]
extern crate test;
extern crate euler_utils;
use euler_utils::prime;

fn main() {
	let mut primes_on_diagonals = 0;
	for len_spir in (3..).step_by(2) {
		let last_diag = len_spir*len_spir;
		let step_size = len_spir - 1;
		let first_diag = last_diag - 3*step_size;
		for num in (first_diag..last_diag).step_by(step_size) {
			if prime::is_prime(num as u64) { primes_on_diagonals += 1 }
		}

		let amount_diagonal_nums = 2*(len_spir - 1) + 1;
		if 10*primes_on_diagonals < amount_diagonal_nums {
			println!("{}", len_spir);
			break;
		}
	}
}

#[bench]
fn bench( b: &mut test::Bencher ) {
	b.iter(|| main())
}
