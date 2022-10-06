#![feature(test)]
extern crate test;

fn max ( a: i32, b : i32 ) -> i32 { if a>b {a} else {b} }

fn main() {
	// read at compile time from file
	let input = include_str!("triangles.txt");

	// parse into vector
	let mut numbers = vec![]; // numbers[line][column]
	for line in input.lines_any() {
		let line_vec : Vec<_> = line.split(' ')
			.map(|s| s.parse().unwrap())
			.collect();
		numbers.push(line_vec);
	}

	for line in ( 0..numbers.len() - 1 ).rev() {
		for i in 0..numbers[line].len() {
			numbers[line][i] += max( numbers[line+1][i], numbers[line+1][i+1] );
		}
	}
	println!("Highest sum: {}", numbers[0][0]);
}

#[bench]
fn main_bench(b:&mut test::Bencher) {
	b.iter(|| main());
}
