#![feature(test)]
extern crate test;
extern crate permutohedron;

use permutohedron::LexicalPermutation;

fn calc_num ( digits : &[u64] ) -> u64 {
	let mut num = 0;
	for (i,&digit) in digits.iter().rev().enumerate() {
		num += digit * 10u64.pow(i as u32);
	}
	num
}

fn main() {
	let mut digits = [1,2,3,4,5,6,7,8,9];
	let mut pandigitals = vec![];
	loop {
		pandigitals.push(digits);
		if !digits.next_permutation() {
			break
		}
	}

	pandigitals.sort();

	'pandigital: for pandigital in pandigitals.iter().rev() {
		'length: for length in 1..5+1 {
			let base_num = calc_num(&pandigital[..length]);
			let mut pos = length;
			let mut len_offset = 0;
			for n in 2..9+1 {
				let num = base_num * n;
				if num.to_string().len() > length { len_offset = 1 };
				let new_pos = pos + length + len_offset;

				if new_pos > 9 ||
				   base_num * n != calc_num(&pandigital[pos..new_pos]) {
					continue 'length;
				}

				pos = new_pos;
				if pos == 9 {
					println!("{}, base: {}\t n in [1,{}]", calc_num(&pandigital[..]), base_num, n);
					break 'pandigital;
				}
			}
		}
	}
}

#[bench]
fn bench ( b:&mut test::Bencher){
	b.iter(|| main());
}
