#![feature(step_by)]
#![feature(collections)]
extern crate prime;
use std::collections::BitVec;

fn phi (mut n: u64, factors: Vec<[u64;2]>) -> u64 {
    n * factors.iter().fold(1, |prod, &[p,_]| (p-1)*prod )
    / factors.iter().fold(1, |prod, &[p,_]| p*prod )

    /*let mut is_coprime = BitVec::from_elem( (n+1) as usize, true);
    for &[prime, _] in factors.iter() {
        for multiple in (prime..n+1).step_by(prime) {
            is_coprime.set(multiple as usize, false);
        }
    }
    (is_coprime.iter().filter(|&is| is).count() - 1) as u64*/
}

fn are_permutations(mut n1: u64, mut n2: u64) -> bool {
    let (mut digs1, mut digs2) = ([0;10], [0;10]);
    while n1 != 0 {
        digs1[(n1 % 10) as usize] += 1;
        n1 /= 10;
    }
    while n2 != 0 {
        digs2[(n2 % 10) as usize] += 1;
        n2 /= 10;
    }
    digs1 == digs2
}

fn main() {
    let max : u64 = 10_000_000;
    let primes = prime::sieve((max) as usize);
    let mut min_ratio = 2f64;
    let mut min_ratio_pos = 0;

    for i in (2..max).rev() {
        //if i % 10_000 == 0 { println!("{}", i) }
        let factors = prime::prime_factors(i, &primes);
        let phi_i = phi(i, factors);
        if !are_permutations(i, phi_i) { continue }
        let ratio = i as f64 / phi_i as f64;
        if ratio < min_ratio {
            min_ratio = ratio;
            min_ratio_pos = i;
            println!("n = {}, ϕ(n) = {} => {}", i, phi_i, ratio);
            //println!("{}", min_ratio_pos);
        }
    }
    //println!("{}", phi(87109, prime::prime_factors(87109, &primes)));

    println!("{}", min_ratio_pos);
}
