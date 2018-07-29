/// Calculate prime numbers up to given upper bound
/// Sieve of Erasthothenes
fn primes_up_to ( max_prime : usize ) -> Vec<u64> {
	let mut is_prime = vec![true; max_prime+1];
	is_prime[0]=false;
	is_prime[1]=false;
	let mut prime_numbers = vec::<u64>![];
	
	for number in 0..is_prime.len() {
		if is_prime[number] {
			prime_numbers.push(number as u64);
			for i in std::iter::range_step(2*number, max_prime, number) { is_prime[i] = false }
		}
		// if number % 1000000 == 0 { println!("{}", number) };
	}
	prime_numbers
}

/// Calculate prime factors by trial division and return vector of results
/// Multiplicities of prime factors not counted
fn prime_factors ( to_factorise : u64, primes : &[u64] ) -> Vec<u64> {
	let mut prime_factors = vec::<i64>![];
	let mut tmp_to_factorise = to_factorise;
	for &prime in primes.iter() {
		if tmp_to_factorise % prime == 0 {
			prime_factors.push(prime);
			while tmp_to_factorise % prime == 0 { tmp_to_factorise /= prime; };
			if tmp_to_factorise == 1 { break }
		}
	}
	prime_factors
}

fn main() {
	println!("Calculating primes");
	let max_prime = 2000000;
	let prime_numbers = primes_up_to(max_prime);
	let consecutives = 4;
	let nr_distinct_primes = 4;
	println!("Primes calculated");
	
	let mut consecutive_distinct_prime = 0;
	for int in 2*3*5*7..max_prime*max_prime-1 {
		if int % 10000 == 0 { println!("Checked up to: {}", int); };
	
		let prime_facts = prime_factors(int as u64, &prime_numbers);
		if prime_facts.len() == nr_distinct_primes {
			consecutive_distinct_prime += 1;
			if consecutive_distinct_prime == consecutives {
				println!("Result: {}", int-3);
				break;
			}
		} else { 
			consecutive_distinct_prime = 0;
		}	
	}
}