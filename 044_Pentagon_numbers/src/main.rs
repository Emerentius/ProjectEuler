#![feature(test)]
extern crate test;
use std::collections::HashSet;

fn min(a:u64,b:u64) -> u64 { if a < b { a } else { b } }

fn is_pentagonal( num: u64 , n: u64 ) -> bool {
	// check upwards of the nth pentagonal
	let mut pentagonal = n*(3*n - 1) / 2;
	let mut next_step = 3*n+1;
	loop {
		pentagonal += next_step;
		next_step += 3;
		if num == pentagonal { return true; }
		if num < pentagonal { return false; }
	}
	//let m = (1 + ((1 + 24*num) as f64).sqrt().round() as u64) / 6;
	//return (3*m*m - m)/2 == num
}

fn main() {
	let mut pentagonal_set = HashSet::new();
	let mut pentagonal_numbers = vec![];
	let mut lowest_diff : u64 = !0;
	let mut pentagonal : u64 = 0;
	let mut next_step : u64 = 1;
	for n in 1.. {
		// nth pentagonal, always
		pentagonal += next_step;
		next_step += 3;
		
		pentagonal_numbers.push(pentagonal);
		pentagonal_set.insert(pentagonal);
		
		for &smaller_pentagonal in pentagonal_numbers.iter().rev() {
			// can also be equally big
			let diff = pentagonal - smaller_pentagonal;
			if diff > lowest_diff { break }
			if pentagonal_set.contains (&diff) 
			&& is_pentagonal(pentagonal+smaller_pentagonal, n as u64) {
				lowest_diff = min(lowest_diff, diff);
			}
		}
		
		if lowest_diff < next_step { break }
	}
	println!("Lowest difference is: {}", lowest_diff);
}

#[bench]
fn bench ( b: &mut test::Bencher ) {
	b.iter(|| main() )
}