extern crate prime;
use std::num::Int;

fn main() {
    let primes = prime::sieve(10_000_000/2);
    let log_target = 7.;
    let target = 10_000_000; //.pow(7);

    let mut sum = 0;
    for (i,&prime1) in primes.iter().enumerate() {
        if i % 10_000 == 0 { println!("{}: {}", i, prime1) }
        //let log1 = (prime1 as f64).log10();
        for &prime2 in &primes[..i] {
            //let mut max_num = 0.;
            let mut max_num = 0;
            let log2 = (prime2 as f64).log10();
            //let (mut co1, mut co2) = (0, (log_target/log2) as u64 + 1);
            let (mut co1, mut co2) = (0, (log_target/log2) as u64 + 1 );

            'c1: for c1 in (1..).take_while(|&c| c*prime1 <= target )  {//(c as f64)*log1 <= log_target) {
                'c2: for c2 in (1..co2).rev() {
                    //let num = (c1 as f64)*log1 + (c2 as f64)*log2;
                    let num = prime1.pow(c1 as u32) + prime2.pow(c2 as u);
                    //if num > log_target { continue }
                    if num > target { continue }
                    if num > max_num {
                        //println!("{}", num);
                        max_num = num;
                        co1 = c1;
                        co2 = c2;
                    }
                    //if num < log_target { continue 'c1 }
                    if num < target { continue 'c1 }
                }
            }
            sum += co1*prime1 + co2*prime2;
        }
    }
    println!("{}", sum);
}
