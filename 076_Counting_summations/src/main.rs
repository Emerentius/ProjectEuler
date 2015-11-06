#![feature(test)]
extern crate test;

fn n_possible_summations(num:u64, primes:&[u64] ) -> u64 {
    match num {
        0 => 1,
        _ => {
            primes.iter()
                .take_while(|p| **p <= num)
                .enumerate()
                .fold(0, |sum, (i, &prime)| {
                    sum + n_possible_summations(num-prime, &primes[i..])
                })
        },
    }
}

fn main() {
    let nums : Vec<_> = (1..101).collect();
    let n = n_possible_summations(100, &nums[..]) - 1;
    println!("{}", n);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
