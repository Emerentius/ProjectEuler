extern crate test;
extern crate prime;
use std::num::Int;

fn main() {
	let primes = prime::sieve(10_000);
	let mut prime_perm_seq = vec![];
	'nxt_prime:for &prime in primes.iter() {
		if prime < 1000 { continue };
		
		// parse digits
		let mut digits : Vec<u64> = vec![];
		for grapheme in prime.to_string().graphemes(true).rev() {
			digits.push( grapheme.parse().unwrap() )
		}
		
		// permute digits and find primes
		// write into vector
		let mut prime_perm_seq_temp = vec![];
		for perm in digits.permutations() {
			let mut num = 0;
			for (i,&digit) in perm.iter().enumerate() {
				num += digit * 10.pow(i)
			}
			if num < 1000 { break };
			if prime::is_prime(num, &primes) { prime_perm_seq_temp.push(num) }
		}
		
		// sort and deduplicate
		// no clue how those duplicates even come in
		// but they do and this kills them
		prime_perm_seq_temp.sort();
		prime_perm_seq_temp.dedup();
		
		// find sequence of constant difference between term
		let mut reduced_seq = vec![];
		'i:for i in 0..prime_perm_seq_temp.len() {
			for j in i+1..prime_perm_seq_temp.len() {
				let per_prime1 = prime_perm_seq_temp[i];
				let per_prime2 = prime_perm_seq_temp[j];
				reduced_seq.push(per_prime1);
				reduced_seq.push(per_prime2);
				
				for &per_prime3 in prime_perm_seq_temp.iter() {
					if per_prime3 - per_prime2 == per_prime2 - per_prime1 {
						reduced_seq.push(per_prime3);
						break 'i;
					}
				}
				reduced_seq.drain();
			}
		}
		
		if reduced_seq.len() == 3 && !prime_perm_seq.contains(&reduced_seq) {
			prime_perm_seq.push(reduced_seq)
		}
		if prime_perm_seq.len() == 2 { break }; // all sequences found
	}
	for seq in prime_perm_seq {
		println!("{:?} => {}{}{}", seq, seq[0], seq[1], seq[2]);
	}
}

#[bench]
fn bench (b:&mut test::Bencher) {
	b.iter(|| main() );
}