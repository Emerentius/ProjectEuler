#![feature(test)]
extern crate test;

extern crate euler_utils;
use euler_utils::prime; // my personal prime library
extern crate primal; // a better prime library

use prime::FareySequence;

const N: usize = 120_000;

fn walk_factorisations(num: usize, rad: usize, primes: &[usize], rads: &mut [usize; N]) {
    for (i, &prime) in primes.iter().enumerate() {
        let mut new_num = num * prime;
        if new_num > N { return }
        let new_rad = rad * prime;

        while new_num <= N {
            rads[new_num-1] = new_rad;
            walk_factorisations(new_num, new_rad, &primes[i+1..], rads);
            new_num *= prime;
        }
    }
}

fn main() {
    let primes = primal::Primes::all().take_while(|&pr| pr <= N).collect::<Vec<_>>();
    let mut rad = [1; N];
    walk_factorisations(1, 1, &primes, &mut rad);

    let solution = FareySequence::new(N)
        .map(|(a, b)| (a, b, a+b))
        .filter(|&(a, b, c)| c < N && rad[a-1]*rad[b-1]*rad[c-1] < c)
        .fold(0, |sum, (_, _, c)| sum + c);
    println!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
