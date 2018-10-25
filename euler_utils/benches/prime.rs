#![feature(test)]

extern crate test;
extern crate euler_utils;
use euler_utils::prime;

#[bench]
fn erat_sieve(b: &mut test::Bencher) {
    b.iter(|| prime::erat_sieve(1_000_000));
}

#[bench]
fn iter_sieve(b: &mut test::Bencher) {
    b.iter(|| prime::prime_iter(1_000_000).count())
}

#[bench]
fn atkin_sieve(b: &mut test::Bencher) {
    b.iter(|| prime::atkin_sieve(1_000_000));
}

#[bench]
fn trial_division_with_primes (b: &mut test::Bencher) {
    let max = 100_000;
    let primes = prime::sieve(max);
    b.iter(|| {
        let mut is_prime = [true; 500_000];
        is_prime[0] = false;
        is_prime[1] = false;
        for (n, is_pr) in is_prime.iter_mut().enumerate() {
            *is_pr = prime::is_prime_trial_div(n as u64, &primes);
        }
    })
}

#[bench]
fn trial_division_without_primes (b: &mut test::Bencher) {
    b.iter(|| {
        let mut is_prime = [true; 500_000];
        is_prime[0] = false;
        is_prime[1] = false;
        for (n, is_pr) in is_prime.iter_mut().enumerate() {
            *is_pr = prime::is_prime_trial_div(n as u64, &[]);
        }
    })
}

#[bench]
fn miller_rabin_primality_test (b: &mut test::Bencher) {
    b.iter(|| {
        let mut is_prime = [true; 500_000];
        is_prime[0] = false;
        is_prime[1] = false;
        for (n, is_pr) in is_prime.iter_mut().enumerate() {
            *is_pr = prime::is_prime(n as u64);
        }
    })
}
