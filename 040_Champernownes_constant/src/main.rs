use std::num::Int;

fn main() {
	let mut string = "0.".to_string();
	for int in 1..1000000+1 { string.push_str( &int.to_string() ) }
	
	let mut product = 1;
	for i in 1..6+1 {
		let position = 10.pow(i) + 1; 
		let digit = string[position..position+1].parse().ok().expect("");
		product *= digit;
	}
	println!("{}", product);
}
