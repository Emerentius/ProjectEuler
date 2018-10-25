extern crate euler_utils;
use euler_utils::prime;
use std::cmp::max;

const LIMIT : u64 = 10_000_000;
const LIM_PLUS : u64 = 10_000_001;

fn next_power_of_ten(num: u64) -> u64 {
    let mut power = 1;
    while power <= num {
        power *= 10;
    }
    power
}

fn insert_chain(new_num: u64, max_in_chain: u64, highest_in_chain: &mut Vec<Option<u64>>) -> bool {
    // returns true if new chain or lower chain was inserted
    let new_max = max(new_num, max_in_chain);

    if (highest_in_chain[new_num as usize]).is_none()
    || Some(new_max) < highest_in_chain[new_num as usize] {
        highest_in_chain[new_num as usize] = Some(new_max);
        true
    } else {
        false
    }
}

fn find_new_relatives(highest_in_chain: &mut Vec<Option<u64>>, is_prime: &prime::IsPrime, primes: &[u64]) -> bool {
    // new or with lower maxima
    let mut found_chains = false;
    for &num in primes {
        if let Some(max_in_chain) = highest_in_chain[num as usize] {

            // find connected primes
            let next_pow = next_power_of_ten(num);
            let mut pow_num = next_pow; // work variable

            if next_pow >= LIMIT { pow_num /= 10 }
            'pow: while pow_num != 0 {
                let digit = (num / pow_num) % 10;
                let num_template = num - pow_num*digit;

                // should have checked here for leading zeros when cutting first digit
                // no difference in result though
                for new_digit in 0..9+1 {
                    let new_num = num_template + pow_num*new_digit;
                    if is_prime[new_num as usize] {
                        found_chains = insert_chain(new_num, max_in_chain, highest_in_chain) || found_chains;
                    }
                }
                pow_num /= 10;
            }
            
        }
    }
    found_chains
}

fn main() {
    // minimal maximum in chain
    let mut highest_in_chain = vec![None; LIM_PLUS as usize];
    highest_in_chain[2] = Some(2);
    let is_prime = prime::prime_iter(LIMIT as usize).to_sieve();
    let primes = prime::sieve(LIMIT as usize);

    // side_effects, loops until no new findings
    while find_new_relatives(&mut highest_in_chain, &is_prime, &primes) {}

    let mut sum = 0;
    for &prime in &primes {
        if highest_in_chain[prime as usize] == None
        || highest_in_chain[prime as usize] > Some(prime) {
            sum += prime;
        }
    }

    println!("{}", sum);
}
