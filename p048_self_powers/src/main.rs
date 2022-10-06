fn main() {
	let mut last_digits : u64 = 0; //: [u32;10] = [0;10];
	let lower_than : u64 = 1_000_000_000_0;
	for i in 1..1000+1 {
		let mut temp : u64 = i;
		for j in 1..i {
			if j % 3 == 0 { temp = temp % lower_than; } // last 10 digits
			temp *= i;
		}
		last_digits += temp % lower_than;
	}
	last_digits = last_digits % lower_than;
	println!("{}", last_digits);
}
