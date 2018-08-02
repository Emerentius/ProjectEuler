#![feature(test)]
#![feature(step_by)]
extern crate test;

fn reverse ( mut number : u32, base : u32 ) -> u32 {
	let mut reversed_number = 0;
	while number != 0 {
		reversed_number = reversed_number * base + (number % base);
		number /= base;
	}
	reversed_number
}

#[inline]
fn is_palindrome (number : u32, base : u32 ) -> bool {
	reverse(number, base) == number
}

fn main() {
	let sum = (1..1_000_000+1).step_by(2)
		.filter(|&num| is_palindrome(num, 10) && is_palindrome(num, 2) )
		.fold(0, |sum, num| sum+num );
	println!("{}", sum);
}

#[bench]
fn bench(b:&mut test::Bencher) {
	b.iter(|| main() )
}
