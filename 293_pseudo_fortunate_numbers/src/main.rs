#![feature(test)]
extern crate test;
extern crate primal;
use primal::is_prime; // deterministic Miller-Rabin
use std::collections::BTreeSet;

fn find_pseudo_fort(admissible: usize) -> usize {
    (admissible+2..).find(|&n| is_prime(n as u64)).unwrap() - admissible
}

// generate all admissibles and find pseudo fortunate nums
fn generate(prod: usize, primes: &[usize], pseudo_fort: &mut BTreeSet<usize>) {
    if prod > N { return }
    pseudo_fort.insert( find_pseudo_fort(prod) );
    generate(prod*primes[0], primes, pseudo_fort);
    generate(prod*primes[1], &primes[1..], pseudo_fort);
}

const N: usize = 1_000_000_000;

fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut pseudo_fort = BTreeSet::new();
    // find all pseudo_forts
    generate(2, &primes, &mut pseudo_fort);
    // sum
    println!("{}", pseudo_fort.iter().fold(0, std::ops::Add::add));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
