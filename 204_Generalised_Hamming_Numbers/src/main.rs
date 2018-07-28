extern crate primal;
extern crate bit_vec;
use bit_vec::BitVec;

const MAX : usize = 1_000_000_000;

fn main() {
    let primes = primal::Primes::all().take_while(|&pr| pr < MAX)
        .collect::<Vec<_>>();
    let mut hamming_numbers = BitVec::from_elem(MAX+1, true);
    //zero doesn't count
    hamming_numbers.set(0, false);

    for prime in primes.into_iter().skip_while(|&p| p<100) {
        for n in (prime as usize..MAX).step_by(prime as usize) {
            hamming_numbers.set(n, false);
        }
    }
    let count = hamming_numbers.iter()
        .filter(|&is_h| is_h)
        .count();
    println!("{}", count);
}
