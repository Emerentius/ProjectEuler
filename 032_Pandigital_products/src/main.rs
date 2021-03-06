#![feature(test)]
extern crate test;
extern crate permutohedron;
use permutohedron::LexicalPermutation;
use std::collections::HashSet;

fn num_from_digits ( digits : &[u64] ) -> u64 {
	let mut num = 0;
	for (i,digit) in digits.iter().enumerate() {
		num += digit * 10u64.pow(i as _);
	}
	num
}

fn main() {
	let mut sum = 0;
	let mut pandigital_products = HashSet::new();

	// 1 or 2 digits, minimise calculating a product twice
	for n_digits1 in 1..3 {
		 // max 9 digits, +1 for inclusiveness, -1 so product's not empty
		for n_digits2 in 1..9+1 - 1 - n_digits1 {
			 // (n+m-1 digits) <= (n-digits)*(m-digits) <= (n+m digits)
			 // and m1_digits + m2_digits + product_digits = 9
			if n_digits1 + n_digits2  > 5 { break; }
			if n_digits1 + n_digits2 < 5 { continue; } // 2n1 + 2n2 < 9 => n1+n2 < 4.5 < 5

			let mut digits = [1,2,3,4,5,6,7,8,9];
			loop {
				let mult1 = num_from_digits(&digits[..n_digits1]);
				let mult2 = num_from_digits(&digits[n_digits1..n_digits1+n_digits2]);
				let product = num_from_digits(&digits[n_digits2+n_digits1..]);

				if mult1*mult2 == product && pandigital_products.insert(product) {
					// only if product wasn't already in set
					sum += product;
				}

				if !digits.next_permutation() {
					break;
				}
			}
		}
	}
	println!("sum1 = {}", sum);
}

#[bench]
fn bench(b:&mut test::Bencher) {
	b.iter(|| main());
}
