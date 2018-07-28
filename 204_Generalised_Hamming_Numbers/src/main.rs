#![feature(collections)]
#![feature(step_by)]
extern crate prime;
use std::collections::BitVec;

fn main() {
    const max : usize = 1_000_000_000;
    let primes = prime::sieve(max);
    let mut hamming_numbers = BitVec::from_elem(max+1, true);
    //zero doesn't count
    hamming_numbers.set(0, false);

    for prime in primes.into_iter().skip_while(|&p| p<100) {
        for n in (prime as usize..max).step_by(prime as usize) {
            hamming_numbers.set(n, false);
        }
    }
    let count = hamming_numbers.iter()
        .filter(|&is_h| is_h)
        .count();
    println!("{}", count);
}
