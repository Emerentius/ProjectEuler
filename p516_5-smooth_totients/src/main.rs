#![feature(test)]
extern crate test;

extern crate primal;
use primal::is_prime;

const LIMIT: u64 = 1_000_000_000_000;
const MOD: u64 = 4294967296; // 2^32;

fn gen_primes(num: u64, primes: &[u64], storage: &mut Vec<u64>) {
    if num > LIMIT { return }
    if is_prime(num+1) { storage.push(num+1); }
    for (i, &pr) in primes.iter().enumerate() {
        gen_primes(num * pr, &primes[i..], storage)
    }
}


fn compute_s(num: u64, primes: &[u64]) -> u32 {
    if 2*num % MOD == 0 { return num as u32 }
    let mut sum = num as u32;
    for (mut i, &pr) in primes.iter().enumerate() {
        if pr <= 5 { i -= 1 }
        if num > LIMIT/pr { break }
        sum += compute_s(num*pr, &primes[i+1..]);
    }
    sum
}

fn main() {
    let mut primes = vec![]; // all primes which can result in hamming nums
    gen_primes(1, &[2, 3, 5], &mut primes);
    primes.sort();
    println!("{}", compute_s(1, &primes));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
