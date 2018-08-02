#![feature(test)]
extern crate test;
extern crate primal;
use primal::Primes;

const N: usize = 10_000_000_000_000_000;
const N_PR: usize = 100;

// n choose n-4, hardcoding is a poor man's memoization
const BINOM: [usize; 16] = [0, 0, 0, 0, 1, 4, 10, 20, 35, 56, 84, 120, 165, 220, 286, 364];

fn count_nums(n_primes: usize, prime_prod: usize, primes: &[usize]) -> usize {
    primes.iter()
        .map(|pr| pr*prime_prod)
        .take_while(|&new_prod| new_prod <= N)
        .enumerate()
        .fold(N/prime_prod * BINOM[n_primes], |cnt, (i, new_prod)| {
            cnt - count_nums(n_primes+1, new_prod, &primes[i+1..])
        })
}

fn main() {
    let primes: Vec<_> = Primes::all().take_while(|&p| p < N_PR).collect();
    println!("{}", count_nums(0, 1, &primes[..]));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
