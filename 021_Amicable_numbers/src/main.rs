#![feature(test)]
extern crate test;

fn prop_sum (nr : i32) -> i32 {
	let mut proper_sum = 0;
	for i in 1..nr/2+1 {
		if nr%i == 0 { proper_sum += i } 
	};
	proper_sum
}

fn main() {
	let mut amicable_numbers = vec![];
	for x in 1..10000 {
		let a = prop_sum(x);
		if prop_sum(a) == x && a != x { amicable_numbers.push(x) }
	};
	
	let mut amic_sum = 0;
	for am_nr in amicable_numbers {	amic_sum += am_nr };
	println!("{}", amic_sum);
}

#[bench]
fn bench (b: &mut test::Bencher) {
	b.iter(|| main())
}