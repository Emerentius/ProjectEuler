extern crate primal; // v0.2
extern crate fnv; // v1.0
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;
use std::collections::HashSet;

// custom hasher, just playing around with optimisations
// not necessary at all
// shaves off ~1s (15-20%)
type MyHasher = BuildHasherDefault<FnvHasher>;
type Set<T> = HashSet<T, MyHasher>;

const PRIME_LIMIT: usize = 47;
const LIMIT: usize = 10_000_000_000_000;

fn generate_smooth_numbers(num: usize, primes: &[usize], bucket: &mut Set<usize>) {
    for (i, &pr) in primes.iter().enumerate() {
        let mut new_num = num * pr;
        if new_num >= LIMIT { break }
        while new_num < LIMIT {
            bucket.insert(new_num);
            generate_smooth_numbers(new_num, &primes[i+1..], bucket);
            new_num *= pr;
        }
    }
}

fn main() {
    let primes = primal::Primes::all()
        .take_while(|&pr| pr <= PRIME_LIMIT)
        .collect::<Vec<_>>();
    let mut smooth_nums = Set::default();
    smooth_nums.insert(1);
    generate_smooth_numbers(1, &primes, &mut smooth_nums);

    let mut sum = 0;
    for &n1 in smooth_nums.iter() {
        if smooth_nums.contains(&(n1 + 1)) {
            sum += n1;
        }
    }
    println!("{}", sum);
}
