#![feature(test)]
extern crate test;
extern crate primal;

const LIMIT: usize = 20_000_000;

fn main() {
    let mut prime_fact_sum = 0;
    for pr in primal::Primes::all().take_while(|&pr| pr <= LIMIT) {
        for pr_pow in (1..).map(|n| pr.pow(n))
            .take_while(|&pr_pow| pr_pow <= LIMIT)
        {
            prime_fact_sum += (LIMIT/pr_pow - 15_000_000/pr_pow - 5_000_000/pr_pow) * pr;
        }
    }
    println!("{}", prime_fact_sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
