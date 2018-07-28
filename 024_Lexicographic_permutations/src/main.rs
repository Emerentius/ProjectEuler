#![feature(collections)]
#![feature(test)]
extern crate test;

fn main() {
	let mut digits : [u64; 10]= [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	for _ in 0..1_000_000-1 { digits.next_permutation(); }
	let num = digits.iter().fold(0, |num, &dig| num * 10 + dig);
	println!("{}", num);
}

#[bench]
fn bench (b: &mut test::Bencher) {
	b.iter(|| main())
}
