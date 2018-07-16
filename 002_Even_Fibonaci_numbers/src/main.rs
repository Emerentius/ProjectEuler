#![feature(test)]
extern crate test;

fn main() {
	let mut fibonacci_numbers = vec![1,1];
	let mut i = 0;
	let mut sum_of_even = 0;
	let target = 4000000;
	let mut next_number;
	
	loop {
		next_number = fibonacci_numbers[i] + fibonacci_numbers [i+1];
		if next_number < target {
			fibonacci_numbers.push( next_number );
			if next_number % 2 == 0 { sum_of_even += next_number };
			i += 1;
		}
		else { break; }
	}
	print!("{}", sum_of_even);
}

#[bench]
fn bench(b: &mut test::Bencher) {
	b.iter(|| main());
}