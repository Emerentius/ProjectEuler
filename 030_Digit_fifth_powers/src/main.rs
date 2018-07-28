extern crate test;
use std::num::Int;

fn digit_power_sum ( number : u64 ) -> u64 {
	let string = number.to_string();
	let mut sum :u64 = 0;
	for grapheme in string.graphemes(true) {
		sum += grapheme.parse::<u64>().unwrap().pow(5);
	}
	sum
}

fn main() {
	let upper_bound = 354294; // 354294 = 6 * 9^5 < 999_999
	
	let mut sum = 0;
	for i in 10..upper_bound {
		if i == digit_power_sum(i) { sum += i }
	}
	println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) { 
	b.iter(|| main());
}