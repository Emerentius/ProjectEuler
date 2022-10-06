#![feature(test)]
extern crate test;
extern crate permutohedron;
use permutohedron::LexicalPermutation;

fn main() {
	let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	for _ in 0..1_000_000-1 { digits.next_permutation(); }
	let num = digits.iter().fold(0, |num, &dig| num * 10 + dig as u64);
	println!("{}", num);
}

#[bench]
fn bench (b: &mut test::Bencher) {
	b.iter(|| main())
}
