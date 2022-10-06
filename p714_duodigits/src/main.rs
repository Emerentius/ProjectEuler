use std::collections::{HashMap, HashSet};

// visit all numbers < `max` that divide the number from which the prime factors were computed
fn walk_divisors(
    factors: &[(usize, usize)],
    max: usize,
    mut num: usize,
    callback: &mut impl FnMut(usize),
) {
    if num > max {
        return;
    }

    callback(num);

    if let Some((&(prime, multiplicity), rest)) = factors.split_first() {
        walk_divisors(rest, max, num, callback);

        for _ in 0..multiplicity {
            num *= prime;
            walk_divisors(rest, max, num, callback);
        }
    }
}

// Recursively generate all duodigit numbers up to `max_depth` length.
// For each number, find their divisors < MAX_PRIME and store the current
// number as least multiple if it's smaller than the previous one.
fn duodigits(
    digit1: usize,
    digit2: usize,
    max_depth: u8,
    sieve: &primal::Sieve,
    max_prime: usize,
    min_multiple: &mut HashMap<usize, u128>,
    nums_left: &mut Vec<usize>,
    nums_found: &mut HashSet<usize>,
    depth: u8,
    num: u128,
) {
    if depth > max_depth {
        return;
    }

    debug_assert!(digit1 < 10);
    debug_assert!(digit2 < 10);

    if num != 0 {
        let mut store_if_lower = |divisor| {
            nums_found.insert(divisor);
            min_multiple
                .entry(divisor)
                .and_modify(|old_min| *old_min = std::cmp::min(*old_min, num))
                .or_insert(num);
        };

        // Early on, find divisors through the prime factorization.
        // Later, go through only the numbers which haven't yet found a duodigit multiple.
        //
        // I started out with the first approach but it slowed down considerably towards
        // the end. The second approach is faster towards the end but slower at first.
        if max_depth <= 12 {
            let mut factors = match sieve.factor(num as usize) {
                Ok(factors) => factors,
                Err((_rest, factors)) => factors,
            };
            factors.retain(|&(prime, _)| prime <= max_prime);

            walk_divisors(&factors, MAX_PRIME, 1, &mut store_if_lower);
        } else {
            for &divisor in nums_left.iter().filter(|&&div| num % div as u128 == 0) {
                store_if_lower(divisor);
            }
        }
    }

    let num1 = num * 10 + digit1 as u128;
    let num2 = num * 10 + digit2 as u128;

    duodigits(
        digit1,
        digit2,
        max_depth,
        sieve,
        max_prime,
        min_multiple,
        nums_left,
        nums_found,
        depth + 1,
        num1,
    );
    duodigits(
        digit1,
        digit2,
        max_depth,
        sieve,
        max_prime,
        min_multiple,
        nums_left,
        nums_found,
        depth + 1,
        num2,
    );
}

const MAX_PRIME: usize = 50_000;
fn main() {
    let sieve = primal::Sieve::new(MAX_PRIME);

    let mut min_multiple = HashMap::new();
    min_multiple.insert(1, 1);

    let mut nums_left = (2..=MAX_PRIME).collect::<Vec<_>>();
    let mut nums_found = HashSet::new();
    nums_found.insert(1);

    // iteratively deepening DFS
    for max_depth in 1.. {
        assert_eq!(MAX_PRIME - min_multiple.len(), nums_left.len());

        // I noticed that the last remaining numbers whose duodigit multiples have yet to be found
        // tend to be multiples of 10.
        // those MUST end in 10 and so does their multiple.
        // x5 speedup towards the end of the search.
        let max_digit1 = if nums_left.iter().all(|n| n % 10 == 0) {
            0
        } else {
            9
        };

        if min_multiple.len() == MAX_PRIME {
            break;
        }
        println!(
            "depth: {:2}, {:5}/{} found",
            max_depth,
            min_multiple.len(),
            MAX_PRIME
        );
        for (digit1, digit2) in (0..max_digit1 + 1)
            .flat_map(|digit1| (digit1 + 1..10).map(move |digit2| (digit1, digit2)))
        {
            duodigits(
                digit1,
                digit2,
                max_depth,
                &sieve,
                MAX_PRIME,
                &mut min_multiple,
                &mut nums_left,
                &mut nums_found,
                0,
                0,
            );
        }

        nums_left.retain(|num| !nums_found.contains(num));
    }

    let solution = min_multiple.values().sum::<u128>();
    println!("{:.12e}", solution as f64);
}
