#![feature(test)]
extern crate test;
extern crate primal;

const LIMIT: u64 = 100_000_000;
const MOD: u64 = 1_000_000_009;

fn pow_mod(mut base: u64, mut exp: u32, modu: u64) -> u64 {
    let mut out = 1;
    while exp != 0 {
        if exp % 2 == 1 {
            out = (out*base) % modu;
        }
        exp /= 2;
        base = (base*base) % modu;
    }
    out
}

// count prime multiplicity in LIMIT!
fn count_occurences(prime: u64) -> u64 {
    let mut limit = LIMIT;
    let mut occ = 0;
    while limit != 0 {
        limit /= prime;
        occ += limit;
    }
    occ
}

fn main() {
    let sum = primal::Primes::all()
        .map(|pr| pr as u64)
        .take_while(|&pr| pr < LIMIT)
        .map(|pr| (pr, count_occurences(pr)) )
        .map(|(pr, occ)| pow_mod(pr, 2*occ as u32, MOD))
        .fold(1, |sum, new_factor| sum * (new_factor+1) % MOD);
    println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
