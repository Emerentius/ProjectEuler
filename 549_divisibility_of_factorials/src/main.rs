#![feature(step_by)]
extern crate primal;

use std::collections::BTreeMap;
use std::cmp::max;

const LIMIT: usize = 100_000_000;
/*
use std::collections::BTreeMap;

/*
fn n_divisors(mut num: usize, divisor: usize) -> usize {
    let mut count = 0;
    while num != 0 {
        num /= divisor;
        count += num;
    }
    count
}
*/
/*
fn s(num: u32, primes: &primal::Sieve) -> u32 {
    let factors = primes.factor(num).unwrap();
    let mut min_m = 0;
    for (prime, mut occurences) in factors.into_iter()
        .map(|(pr, occ)| (pr as u32, occ as u32))
    {
        let mut min_m_for_prime = prime * occurences;
        while occurences != 0 {
            occurences /= prime;
            min_m_for_prime -= occurences;
        }
        min_m = std::cmp::max(min_m, min_m_for_prime);
    }
    min_m
}
*/
fn s<T: IntoIterator<Item=(u32, u32)>>(num: u32, factors: T) -> u32 {
    //let factors = primes.factor(num).unwrap();
    let mut min_m = 0;
    for (prime, mut occurences) in factors.into_iter()
        //.map(|(pr, occ)| (pr as u32, occ as u32))
    {
        let mut min_m_for_prime = prime * occurences;
        while occurences != 0 {
            occurences /= prime;
            min_m_for_prime -= occurences;
        }
        min_m = std::cmp::max(min_m, min_m_for_prime);
    }
    min_m
}

const LIMIT: usize = 1_00_000_000;

fn main() {
    let mut factors = vec![BTreeMap::new(); LIMIT+1];
    for pr in primal::Primes::all().take_while(|&pr| pr < LIMIT) {
        let mut multiple = pr;
        while multiple <= LIMIT {
            for num in (multiple..LIMIT+1).step_by(multiple) {
                *factors[num].entry(pr as u32).or_insert(0u8) += 1;
            }
            multiple *= pr;
        }
    }

    for factor in factors.into_iter().take(10) {
        println!("{:?}", factor);
    }
    /*
    let primes = primal::Sieve::new(10_000);
    let sum = (2..10usize.pow(8)+1)
        .inspect(|&n| if n % 100_000 == 0 { println!("{}", n) } )
        .map(|n| s(n, &primes))
        .fold(0, std::ops::Add::add);
    println!("{}", sum);
    println!("{}", s(10, &primes));
    println!("{}", s(25, &primes));
    */
}
*/
/*
// INCORRECT
fn min_m_for_prime(prime: u32, mut occurences: u32) -> u32 {
    let mut min_m_for_prime = prime * occurences;
    while occurences != 0 {
        occurences /= prime;
        min_m_for_prime -= occurences;
    }
    min_m_for_prime
}

const LIMIT: u32 = 100; //1_00_000_000;

fn main() {
    let mut all_s = vec![0; LIMIT as usize +1];
    let mut occurences = vec![0; LIMIT as usize +1]; // scratch space, reused for every prime

    for prime in primal::Primes::all()
        .map(|pr| pr as u32)
        .take_while(|&pr| pr <= LIMIT)
    {
        let mut multiple = prime;
        while multiple <= LIMIT {
            for num in (multiple..LIMIT+1).step_by(multiple) {
                occurences[num as usize] += 1;
            }
            multiple *= prime;
        }

        if prime == 2 {
            println!("{}", occurences[4]);
            println!("{}", min_m_for_prime(2, 2))
        }

        for (min_m, &occ) in std::iter::Iterator::zip(all_s.iter_mut(), occurences.iter()) {
            *min_m = std::cmp::max(*min_m, min_m_for_prime(prime, occ));
        }

        // reset occurences
        for occ in &mut occurences {
            *occ = 0;
        }
    }
    use std::collections::BTreeMap;
    let mut histogram = BTreeMap::new();

    for &t in all_s.iter() {
        *histogram.entry(t).or_insert(0) += 1;
        println!("{}", t);
    }

    println!("");

    for (prime, count) in histogram {
        println!("{:4} {}", prime, count);
    }

    //let sum = all_s.into_iter().fold(0, std::ops::Add::add);
    //println!("{}", sum);
}
*/

fn count_prime_occ(mut num: usize, prime: usize) -> usize {
    let mut occ = 0;
    while num != 0 {
        num /= prime;
        occ += num;
    }
    occ
}

fn walk_nums(num: usize, primes: &[usize], prime_s: &BTreeMap<usize, Vec<usize>>, s: &mut Vec<usize>) {
    for (i, &pr) in primes.iter().enumerate() {
        //let mut pr_pow = pr;
        let mut new_num = num*pr;
        if new_num > LIMIT {
            break
        }
        for &pr_pow_s in (&prime_s[&pr]).iter() {
            if new_num > LIMIT {
                break
            }
            s[new_num as usize] = max(s[num], pr_pow_s);
            walk_nums(new_num, &primes[i+1..], prime_s, s);
            new_num *= pr;
            /*match num.checked_mul(pr_pow) {
                Some(new_num) if new_num <= LIMIT => {
                    s[new_num as usize] = max(s[num], pr_pow_s);
                    walk_nums(new_num, &primes[i+1..], prime_s, s);
                },
                Some(_) | None => break,
            }
            pr_pow *= pr;
            */
        }
    }
}

fn main() {
    let primes = primal::Primes::all()
        .map(|pr| pr as usize)
        .take_while(|&pr| pr < LIMIT)
        .collect::<Vec<_>>();

    let mut prime_s = BTreeMap::new();
    let mut s = vec![0; LIMIT+1];
    // compute s for prime powers
    for &pr in &primes {
        let mut s_collection = vec![];
        let mut pr_pow = pr;
        let mut pr_s = pr;
        let mut pow = 1;
        while pr_pow <= LIMIT {
            while count_prime_occ(pr_s, pr) < pow {
                pr_s += pr;
            }
            s_collection.push(pr_s);
            s[pr_pow] = pr_s;
            pr_pow *= pr;
            pow += 1;
        }
        prime_s.insert(pr, s_collection);
    }
    //println!("{:?}", prime_s[&2]);

    walk_nums(1, &primes, &prime_s, &mut s);
    println!("{}",
        s.iter().fold(0, |sum, &s| sum + s as u64)
    );
}
