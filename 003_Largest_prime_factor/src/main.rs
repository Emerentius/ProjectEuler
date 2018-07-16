fn main() {
	let to_factorize:i64 = 600851475143;
	let try_up_to:i64 = (to_factorize as f64).sqrt() as i64;
	let mut prime_numbers = vec![2];

	let mut temp : i64;
	let mut not_a_prime = false;
	for x in 3..(try_up_to+1) {
		temp = x;
		let x_root = (x as f64).sqrt() as i64;
		for y in prime_numbers.iter() {
			if temp % *y == 0 {
				not_a_prime = true;
				break;
			}
			if x_root < *y {break;};
		}
		if !not_a_prime { prime_numbers.push(x) }
		not_a_prime = false;

		if x % 10000 == 0 { println!("{}", x) };
	};
	println!("prime numbers calculated.");

	let mut prime_factors = vec![];
	for x in prime_numbers {
		if to_factorize % x == 0 { prime_factors.push(x); }
	};

	let mut multiplication = 1;
	for x in prime_factors.iter() {
		println!("{}, Rest = {}", x, to_factorize % *x);
		multiplication = multiplication * *x;
	};
	println!("{}", multiplication);
}
