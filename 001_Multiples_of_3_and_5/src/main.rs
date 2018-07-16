#![feature(test)]
#![feature(core)]
extern crate test;

fn main() {
	let sum : u32 = (0..1_000).filter(|&n| n%3==0 || n%5==0).sum();
	println!("{}", sum);
}

#[bench]
fn bench (b: &mut test::Bencher) {
	b.iter(|| main())
}
