extern crate euler_utils;
use euler_utils::prime;

fn digit_length(mut num: u64) -> u32 {
    let mut length = 0;
    while num != 0 {
        num /= 10;
        length += 1;
    }
    length
}

fn main() {
    let primes = prime::sieve(1_000_003); // first prime after 1_000_000

    let mut sum = 0;
    for (i,two_primes) in primes.windows(2).skip(2).enumerate() {
        if i % 1000 == 0 { println!("{}", i); }
        if let &[prime1, prime2] = two_primes {
            let len = digit_length(prime1);
            let modu = 10u64.pow(len);
            sum += (2*prime2..).step_by(prime2 as usize).find(|num| num % modu == prime1).unwrap();
        }
    }
    println!("{}", sum);
}
