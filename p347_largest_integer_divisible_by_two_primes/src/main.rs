#![feature(test)]
extern crate test;
extern crate primal;

fn M(prime1: usize, prime2: usize, target: usize) -> usize {
    let mut max_num = 0;
    let mut num = prime1*prime2;
    if num > target { return 0 } // impossible

    // calc p1*p2^(max_exp2) <= target with maximum exp2
    let mut exp2 = 1;
    while num*prime2 <= target {
        num *= prime2;
        exp2 += 1;
    }

    // divide by prime2 repeatedly, correct as much as possible with prime1 at every step
    loop {
        if num > max_num { max_num = num }
        num /= prime2;
        exp2 -= 1;
        if exp2 == 0 { break }
        while num*prime1 <= target { num *= prime1 }
    }

    max_num
}

fn main() {
    let target = 10_000_000;
    let primes = primal::Primes::all()
        .take_while(|&pr| pr < target/2)
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i,&prime1) in primes.iter().enumerate() {
        for &prime2 in &primes[..i] {
            match M(prime1, prime2, target) {
                0 => break,
                m @ _ => sum += m,
            }
        }
    }
    println!("{}", sum);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
