extern crate smallvec; // v0.3
extern crate num; // v0.1
use smallvec::SmallVec; // small optimization
use num::rational::Ratio;
use std::collections::HashSet;
use std::ops::{Add, Sub, Div, Mul};

type DigitVec = SmallVec<[Ratio<i32>; 9]>;

fn walk_concatenations(pos: usize, mut digits: DigitVec, reachable_nums: &mut HashSet<i64>, seen_before: &mut HashSet<DigitVec>) {
    walk_reachable(digits.clone(), reachable_nums, seen_before);
    if pos < digits.len()-1 {
        // no concatenation
        walk_concatenations(pos+1, digits.clone(), reachable_nums, seen_before);
    }
    for _ in pos..digits.len()-1 {
        // 1+ concatenations
        digits[pos] = digits[pos] * Ratio::new(10, 1) + digits.remove(pos+1);
        walk_concatenations(pos+1, digits.clone(), reachable_nums, seen_before);
    }
}

fn walk_reachable(digits: DigitVec, reachable_nums: &mut HashSet<i64>, seen_before: &mut HashSet<DigitVec>) {
    if digits.len() == 1 && digits[0].is_integer() {
        // everything evaluated and result is integral, save it
        reachable_nums.insert( *digits[0].numer() as i64);
    } else if digits.len() > 2 && !seen_before.insert(digits.clone()) {
        // intermediate result seen before
        // only save earlier results to keep set small
        return
    }

    for i in 0..digits.len()-1 {
        for op in [Add::add, Mul::mul, Sub::sub, Div::div].iter()
            // skip duplicate and disallowed ops on 0
            .take( if *digits[i+1].numer() == 0 {2} else {4} )
        {
            let mut new_digits = digits.clone();
            new_digits[i] = op(new_digits[i], new_digits.remove(i+1));
            walk_reachable(new_digits, reachable_nums, seen_before);
        }
    }
}

fn main() {
    let digits = (1i32..9+1).map(Ratio::from_integer).collect();
    let mut reachable_nums = HashSet::new();
    let mut seen_before = HashSet::new();
    walk_concatenations(0, digits, &mut reachable_nums, &mut seen_before);
    println!("{}", reachable_nums.into_iter().filter(|&num| num > 0).sum::<i64>() );
}
