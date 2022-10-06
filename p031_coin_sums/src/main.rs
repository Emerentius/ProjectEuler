#![feature(test)]
extern crate test;

fn count_combinations(n: usize, rest: u32) -> u32 {
	match n == DENOMINATIONS.len() {
		true if rest == 0 => 1,
		true              => 0,
		false => (0..rest+1).step_by(DENOMINATIONS[n])
			.map(|money| count_combinations(n+1, rest-money))
			.sum(),
	}
}

const DENOMINATIONS: [usize; 8] = [200, 100, 50, 20, 10, 5, 2, 1];

fn main() {
	let variations = count_combinations(0, 200);
	println!("{}", variations)
}

#[bench]
fn bench(b: &mut test::Bencher) { b.iter(|| main() ) }
