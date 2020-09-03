use std::collections::HashMap;

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

fn duodigits(
    digit1: usize,
    digit2: usize,
    max_depth: u8,
    sieve: &primal::Sieve,
    max_prime: usize,
    min_multiple: &mut HashMap<usize, usize>,
    depth: u8,
    num: usize,
) {
    if depth > max_depth {
        return;
    }

    debug_assert!(digit1 < 10);
    debug_assert!(digit2 < 10);

    let mut factors = match sieve.factor(num as usize) {
        Ok(factors) => factors,
        Err((_rest, factors)) => factors,
    };
    factors.retain(|&(prime, _)| prime <= max_prime);

    walk_divisors(&factors, MAX_PRIME, 1, &mut |divisor| {
        min_multiple
            .entry(divisor)
            .and_modify(|old_min| *old_min = std::cmp::min(*old_min, num))
            .or_insert(num);
    });

    let num1 = num * 10 + digit1;
    let num2 = num * 10 + digit2;

    duodigits(
        digit1,
        digit2,
        max_depth,
        sieve,
        max_prime,
        min_multiple,
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
        depth + 1,
        num2,
    );
}

const MAX_PRIME: usize = 5_000;
fn main() {
    let sieve = primal::Sieve::new(MAX_PRIME);

    let mut min_multiple = HashMap::new();
    min_multiple.insert(1, 1);

    // iteratively deepening DFS
    for max_depth in 1.. {
        if min_multiple.len() == MAX_PRIME {
            println!("reached goal, max depth: {}", max_depth - 1);
            break;
        }
        println!("depth: {}, {}/{}", max_depth, min_multiple.len(), MAX_PRIME);
        for (digit1, digit2) in
            (0..10).flat_map(|digit1| (digit1 + 1..10).map(move |digit2| (digit1, digit2)))
        {
            duodigits(
                digit1,
                digit2,
                max_depth,
                &sieve,
                MAX_PRIME,
                &mut min_multiple,
                0,
                0,
            );
        }
    }

    let solution = min_multiple.values().sum::<usize>();

    println!("{:.12e}", solution as f64);
}
