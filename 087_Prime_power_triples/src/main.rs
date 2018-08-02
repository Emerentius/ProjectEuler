#![feature(test)]
extern crate test;
extern crate prime;

fn main() {
    let limit = 5 * 10u64.pow(7);
    let primes = prime::sieve( (limit as f64).sqrt() as usize );

    let mut power_triples = vec![false; limit as usize];

    for &pr4 in primes.iter() {
        // prime power
        let pp4 = pr4.pow(4);
        if pp4 >= limit { break }

        for &pr3 in primes.iter() {
            let pp34 = pr3.pow(3) + pp4;
            if pp34 >= limit { break }

            for &pr2 in primes.iter() {
                let pp234 = pr2*pr2 + pp34;
                if pp234 >= limit { break }
                power_triples[pp234 as usize] = true;
            }
        }
    }
    let count = power_triples.into_iter().filter(|is| *is).count();
    println!("{}", count);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
