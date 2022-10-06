fn factorial ( number : u64 ) -> u64 {
	match number {
		0 | 1 => 1,
		2 => 2,
		3 => 6,
		4 => 24,
		5 => 120,
		6 => 720,
		7 => 5040,
		8 => 40320,
		9 => 362880,
		_ => number*factorial(number-1),
	}
}

fn digit_factorial_sum ( number : u64 ) -> u64 {
	let string = number.to_string();
	let mut sum = 0;
	for i in 0..string.len() {
		let slice = &string[i..i+1];
		sum += factorial(slice.parse().ok().expect(""));
	}
	sum
}

fn main() {
	let upper_bound = 2177281; //9999999 //1854721;
	
	let mut sum = 0;
	for i in 10..upper_bound {
		if i == digit_factorial_sum(i) { sum += i }
	}
	println!("{}", sum);
}