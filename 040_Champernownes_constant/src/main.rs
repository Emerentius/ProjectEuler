fn main() {
	let mut string = "0.".to_string();
	for int in 1..1000000+1 { string.push_str( &int.to_string() ) }

	let mut product = 1;
	for i in 1..6+1 {
		let position = 10usize.pow(i) + 1;
		let digit: u64 = string[position..position+1].parse().ok().expect("");
		product *= digit;
	}
	println!("{}", product);
}
