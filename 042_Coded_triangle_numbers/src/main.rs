extern crate test;
use std::ascii::AsciiExt;

use std::io::prelude::*;
use std::fs::File;
use std::str::StrExt;
use std::iter::IteratorExt;

fn namescore ( word : &str ) -> u32 {
	// all ascii and uppercase, no further checks done
	//if !word.is_ascii() { panic!("Error, non-ascii name: {}", word) };
	let namescore_offset = 'A' as u32 - 1;
	word.chars().fold(0, |sum, char| sum + char as u32 - namescore_offset)
}

fn main() {
	let mut input = String::new();
	
	// File IO
	let path = Path::new(r"D:\Code\Project Euler\042_Coded_triangle_numbers\target\p042_words.txt");
	//Path::new(r".\p042_words.txt");	
	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(err) => panic!("Couldn't open {}: {}", path.display(), err.description()),
	};
	if let Err(err) = file.read_to_string(&mut input) {
		panic!("Couldn't read {}: {}", path.display(), err.description())
	};
	
	input = input.replace("\"", ""); // delete all "
	
	// first 50 triangle numbers
	let triangle_numbers : Vec<_> = (0..50).map(|i| i*(i+1)/2).collect(); 	
	
	/*let count = input.split(",")
			 .filter(|x| triangle_numbers.contains( &namescore(x) ))
			 .count();*/
	let mut count = 0;
	for word in input.split(",") {
		if triangle_numbers.contains( &namescore(word) ) { count += 1 }
	}
	println!("{}", count);
}

#[bench]
fn bench ( b: &mut test::Bencher) {
	b.iter(|| main() );
}

/*
#[bench]
fn without_file_io ( b: &mut test::Bencher) {
	let path = Path::new(r"D:\Code\Project Euler\042_Coded_triangle_numbers\target\p042_words.txt");
	//Path::new(r".\p042_words.txt");	
	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(err) => panic!("Couldn't open {}: {}", path.display(), err.description()),
	};
	let mut input = String::new();
	if let Err(err) = file.read_to_string(&mut input) {
		panic!("Couldn't read {}: {}", path.display(), err.description())
	};
	
	b.iter(|| { 
		input = input.replace("\"", ""); // delete all "
		// first 50 triangle numbers
		let triangle_numbers : Vec<_> = (0..50).map(|i| i*(i+1)/2).collect(); 	
		
		let count = input.split(",")
		 .filter(|x| triangle_numbers.contains( &namescore(x) ))
		 .count();
		/*
		let mut count = 0;
		for word in input.split(",") {
			if triangle_numbers.contains( &namescore(word) ) { count += 1 }
		}
		*/
		println!("{}", count);
	
	});
}*/