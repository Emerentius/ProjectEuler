#![feature(test)]
extern crate test;
use std::collections::BTreeMap;

fn is_palindrome(mut num: u64, cache: &mut Vec<u64>) -> bool {
    cache.clear();
    let mut len = 0;
    while num != 0 {
        let digit = num % 10;
        cache.push(digit);
        len += 1;
        num /= 10;
    }
    (&cache[..len/2]).iter().eq( (&cache[(len+1)/2..len]).iter().rev() )
}

fn square(n: u64) -> u64 { n*n }
fn cube(n: u64)   -> u64 { n*n*n }

fn main() {
    let mut cache = vec![];
    let mut is_palindrome = |num| is_palindrome(num, &mut cache);

    let mut squares = vec![1];
    let mut cubes = vec![1];

    // roots, up to which squares and cubes have already been calculated
    let mut n_sq = 1;
    let mut n_cu = 1;

    // no numbers encountered in the future can be lower than this
    let mut lower_boundary = 1;

    let mut palindrome_sum_count: BTreeMap<u64, Option<u8>> = BTreeMap::new();
    let mut solution_nums = vec![];
    let mut nums_found = 0;

    while nums_found < 5 {
        let (num, other_nums) = if square(n_sq+1) < cube(n_cu+1) {
            n_sq += 1;
            squares.push(square(n_sq));
            (square(n_sq), cubes.iter())
        } else {
            n_cu += 1;
            cubes.push(cube(n_cu));
            (cube(n_cu), squares.iter())
        };

        for num2 in other_nums {
            let sum = num + num2;

            match palindrome_sum_count.get_mut(&sum) {
                None => { // sum never encountered before
                    if is_palindrome(sum) {
                        palindrome_sum_count.insert(sum, Some(1));
                    }
                },
                Some(&mut None) => (),
                Some(&mut Some(old_count)) => {
                    palindrome_sum_count.insert(sum, Some(old_count+1));
                }

            }
        }

        // no numbers encountered in the future can be lower than this
        let next_lower_boundary = std::cmp::min(square(n_sq+1), cube(n_cu+1)) + 1;

        for (num, sum_count) in palindrome_sum_count.range(lower_boundary..next_lower_boundary)
            .flat_map(|(&num, &opt_count)| opt_count.map(|count| (num, count))) // filter out non-palindromes (signified by None)
        {
            if sum_count == 4 {
                nums_found += 1;
                solution_nums.push(num);
            }
        }
        lower_boundary = next_lower_boundary;
    }

    println!("{}", solution_nums.iter().sum::<u64>());
}


#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
