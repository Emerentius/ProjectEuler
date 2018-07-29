extern crate test;
use std::num::Int;

fn main() {
	let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let primes = [2,3,5,7,11,13,17];
	
	let mut sum = 0;
	'perm: for permutation in digits.permutations() {
		if permutation[0] == 0 { continue } // not a pandigital
		for (subslice, &prime) in permutation.windows(3).skip(1)
					  .zip( primes.iter() ) {
			let mut num : u64 = 0;
			for (j,&digit) in subslice.iter().rev().enumerate() {
				num += digit * 10.pow(j as u32);
			}
			
			if num % prime != 0 { continue 'perm }
		}
		let perm_number = permutation.iter().rev().enumerate()
				  .fold(0, |sum, (i,char)| sum + *char * 10.pow(i as u32) );
		sum += perm_number;
	}
	println!("{}", sum);
}

#[bench]
fn bench ( b:&mut test::Bencher) {
	b.iter(|| main());
}