extern crate prime;

// works but so goddamn ugly with all those type casts

fn main() {
	let primes = prime::sieve(1_000_000);
	let mut a_b = (0isize,0isize);
	let mut max_series_length = 0;


	// n=0, P(n) prime => b prime
	for (i,&b) in primes.iter().enumerate() {
		if b > 1000 { break };
		for a in (1 - b as isize)..1001 {
			let mut series_length = 0;

			for n in 0isize.. {
				let pol = (n*n) + a*n + b as isize;

				if pol > 1 && pol == prime::factors(pol as u64, &primes)[0][0] as isize {
					series_length += 1;
				}
				else {
					break;
				}

			}

			if series_length > max_series_length {
				max_series_length = series_length;
				a_b = (a,b as isize);
			}
		}
	}
	println!("{}", max_series_length);
	println!("{} * {} = {}", a_b.0, a_b.1, a_b.0 * a_b.1 );
}
