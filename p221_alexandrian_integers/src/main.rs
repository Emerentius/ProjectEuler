// 64-bit only
extern crate primal; // prime library, v0.2

fn walk_divisors<F: FnMut(usize)>(num: usize, factors: &[(usize, usize)], callback: &mut F) {
    callback(num);
    for (i, &(prime, exp)) in factors.iter().enumerate() {
        let mut new_num = num;
        for _ in 0..exp {
            new_num *= prime;
            walk_divisors(new_num, &factors[i+1..], callback);
        }
    }
}

fn alex_num(p: usize, d: usize) -> usize {
    // hoping the solution fits in 64 bits
    p.saturating_mul(p+d).saturating_mul(p+(p*p+1)/d)
}

fn main() {
    let primes = primal::Sieve::new(40_000_000);
    let mut alexandrian_nums = vec![];
    for p in 1..100_000 {
        let factors = primes.factor(p*p+1).unwrap();
        let mut save_alexandriun_nums = |div| {
            let alex = alex_num(p, div);
            alexandrian_nums.push(alex);
        };
        walk_divisors(1, &factors, &mut save_alexandriun_nums);
    }
    alexandrian_nums.sort();
    alexandrian_nums.dedup();
    println!("{}", alexandrian_nums[150_000-1]);
}
