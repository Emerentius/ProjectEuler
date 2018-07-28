#![feature(test)]
extern crate test;

fn namescore ( word : &str ) -> u32 {
	// all ascii and uppercase, no further checks done
	let namescore_offset = 'A' as u32 - 1;
	word.chars().fold(0, |sum, char| sum + char as u32 - namescore_offset)
}

fn main() {
	let input = include_str!("p042_words.txt").to_string();

	// first 50 triangle numbers
	let triangle_numbers : Vec<_> = (0..50).map(|i| i*(i+1)/2).collect();

	let count = input.split(',')
		.map(|word| word.trim_matches('"'))
		.filter(|word| triangle_numbers.contains(&namescore(word)))
		.count();
	println!("{}", count);
}

#[bench]
fn bench ( b: &mut test::Bencher) {
	b.iter(|| main() );
}
