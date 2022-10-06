#![feature(test)]
extern crate test;

fn reverse(mut number: u32) -> u32 {
	let mut reversed_number = 0;
	while number != 0 {
		reversed_number = reversed_number * 10 + number % 10;
		number /= 10;
	}
	reversed_number
}

fn main() {
	let mut greatest_palindrome = 0;

	for x in (100..1000).rev() {
		for xy in (100..x+1).rev().map(|y| x*y) {
			if xy < greatest_palindrome { break }
			if xy == reverse(xy) { greatest_palindrome = xy }
		}
	}
	println!("{}", greatest_palindrome);
}

#[bench]
fn bench_main(b: &mut test::Bencher) {
	b.iter(|| main());
}
