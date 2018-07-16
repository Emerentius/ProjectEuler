extern crate primal;
use std::cmp::max;

const LIMIT: usize = 100_000_000;

fn count_prime_occ(mut num: usize, prime: usize) -> usize {
    let mut occ = 0;
    while num != 0 {
        num /= prime;
        occ += num;
    }
    occ
}

fn walk_nums(num: usize, s: usize, prime_s: &[(usize, Vec<usize>)], s_coll: &mut Vec<usize>) {
    for (i, &(pr, ref s_of_prime_pows)) in prime_s.iter().enumerate() {
        let mut new_num = num*pr;
        if new_num > LIMIT {
            break
        }
        for &pr_pow_s in s_of_prime_pows.iter() {
            if new_num > LIMIT {
                break
            }
            let new_s = max(s, pr_pow_s);
            s_coll[new_num] = new_s;
            walk_nums(new_num, new_s, &prime_s[i+1..], s_coll);
            new_num *= pr;
        }
    }
}

fn main() {
    let primes = primal::Primes::all()
        .map(|pr| pr as usize)
        .take_while(|&pr| pr < LIMIT)
        .collect::<Vec<_>>();

    let mut prime_s = vec![];
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
            pr_pow *= pr;
            pow += 1;
        }
        prime_s.push((pr, s_collection));
    }

    walk_nums(1, 1, &prime_s, &mut s);
    println!("{}",
        s.iter().fold(0, |sum, &s| sum + s as u64)
    );
}
