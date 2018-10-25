#![feature(test)]
extern crate test;

extern crate primal;
extern crate euler_utils;
use euler_utils::num::multiplicative_inverse;

// sum (p-k), k in 1..5+1
fn sum(prime: i64) -> i64 {
    // (p-1)! mod p = -1
    // => (p-2)! = x2 (mod p), x2*(p-1) = -1 (mod p)
    // => (p-3)! = x3 (mod p), x3*(p-2)*(p-1) = -1 = 2x3 (mod p)
    // and so on, also x_i = (p-(i-1)) * x_(i-1) = 1-i * x_(i-1) (mod p)

    // compute min(num), 0 <= num < prime
    let num = if prime < 24 { (5*prime - 24) % prime } else { prime - 24 }; // get it into the positive
    9*multiplicative_inverse(num, prime).unwrap() % prime
}

fn main() {
    let S = primal::Primes::all()
        .skip_while(|&pr| pr < 5)
        .take_while(|&pr| pr < 100_000_000)
        .map(|pr| sum(pr as i64))
        .fold(0, std::ops::Add::add);
    println!("{}", S);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
