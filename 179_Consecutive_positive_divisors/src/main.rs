#![feature(slice_patterns)]
extern crate prime;

fn main() {
    let primes = prime::sieve(10us.pow(7)/2);
    let mut n_divisors_last = 0;
    let mut count = 0;
    for n in 2..10u64.pow(7) {
        if n % 1_000_000 == 0 { println!("{}", n) }
        let factors = prime::prime_factors(n, &primes);
        let n_divisors = factors.into_iter()
            .fold(1, |prod, [_,occ]| prod * (occ+1));
        if n_divisors == n_divisors_last { count += 1 }
        n_divisors_last = n_divisors;
    }
    println!("{}", count);
}
