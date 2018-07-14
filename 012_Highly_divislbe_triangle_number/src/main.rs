// prime calculation copied from 004
/// Calculate prime numbers up to given upper bound
/// Sieve of Erasthothenes
fn primes_up_to ( max_prime : u64 ) -> Vec<u64> {
	let mut is_prime = vec![true; max_prime as usize +1];
	is_prime[0]=false;
	is_prime[1]=false;
	let mut prime_numbers = vec![];

	for number in 0..is_prime.len() {
		if is_prime[number] {
			prime_numbers.push(number as u64);
			for i in (2*number..max_prime as usize).step_by(number) {
				is_prime[i] = false
			}
		}
		// if number % 1000000 == 0 { println!("{}", number) };
	}
	prime_numbers
}

fn find_prime_occurences (number : i64, prime:&u64) -> (i64, i64) {
	let mut occurences = 0;
	let mut non_divisible_number = number;
	loop {
		if non_divisible_number % *prime as i64 == 0 {
			occurences += 1;
			non_divisible_number = non_divisible_number / *prime as i64;
		} else {
			break;
		}
	}
	(non_divisible_number, occurences)
}

fn main() {
	// calculate prime numbers, upper end just a guess
	let max_prime = 2000000;
	let mut prime_numbers = primes_up_to(max_prime);

	// calculate factors for triangle numbers and factorize them
	// triangle numbers = n*(n+1) / 2
	for n in 1.. {
		// assignment depending on which factor is even, n or (n+1)
		let (mut a, mut b) = (0,0);
		if n % 2 == 0	{
			a = n/2;
			b = n+1;
		} else {
			a = n;
			b = ((n+1)/2);
		}

		let (mut prime_a, mut prime_b) = (vec![], vec![]);
		let (mut temp_a, mut temp_b) = (a,b);
		let mut occ = 0;

		// factorize a and b by trial division
		for prime in prime_numbers.iter() {
			let result_a = find_prime_occurences(temp_a, prime);
			temp_a = result_a.0;
			occ = result_a.1 as _;
			prime_a.push([*prime, occ]);
			//println!("temp_a = {}", temp_a);

			let result_b = find_prime_occurences(temp_b, prime);
			temp_b = result_b.0;
			occ = result_b.1 as _;
			prime_b.push([*prime, occ]);

			if temp_a == 1 && temp_b == 1 { break; }
		}

		if n % 1000 == 0 {println!("{}. triangle number", n)}

		// find sum of prime occurences
		// amount of divisors = (a+1)(b+1)(c+1)...
		// for p^a * q^b * r^c...; p,q,r prime
		let mut sums_prime_occurences = vec![];
			for prime_occ_a in prime_a.iter() {
				for prime_occ_b in prime_b.iter() {
					//let mut occ_sum = 0;
					if prime_occ_a[0] != prime_occ_b[0] { continue; }
					else {
						let occ_sum = prime_occ_a[1]+prime_occ_b[1];
						sums_prime_occurences.push(occ_sum);
						break;
					}
				}
			}

		let mut divisors = 1;
		for occ in sums_prime_occurences {
			divisors = divisors * (occ+1);
		}

		if divisors >= 500 {
			println!(">=500 divisors for {}, the {}. triangle number", a*b, n);
			break;
		}
	}
}
