#![feature(test)]
extern crate test;
extern crate permutohedron;
use permutohedron::LexicalPermutation;

fn permutations(mut digits: [u64; 10]) -> impl Iterator<Item = [u64; 10]> {
	let mut finished = false;
	std::iter::repeat(())
		.scan((), move |(), ()| {
			let next = digits;
			match (digits.next_permutation(), finished) {
				(true, _) => Some(next),
				(false, false) => {
					finished = true;
					Some(next)
				}
				(false, true) => None,
			}
		})
}

fn main() {
	let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let primes = [2,3,5,7,11,13,17];

	let mut sum = 0;
	'perm: for permutation in permutations(digits) {
		if permutation[0] == 0 { continue } // not a pandigital
		for (subslice, &prime) in permutation.windows(3).skip(1)
			.zip( primes.iter() )
		{
			let mut num = 0;
			for (j,&digit) in subslice.iter().rev().enumerate() {
				num += digit * 10u64.pow(j as u32);
			}

			if num % prime != 0 { continue 'perm }
		}
		let perm_number = permutation.iter().rev().enumerate()
				  .fold(0, |sum, (i,ch)| sum + *ch * 10u64.pow(i as u32) );
		sum += perm_number;
	}
	println!("{}", sum);
}

#[bench]
fn bench ( b:&mut test::Bencher) {
	b.iter(|| main());
}
