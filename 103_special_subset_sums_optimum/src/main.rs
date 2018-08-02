#![feature(test)]
extern crate test;

use std::collections::BTreeSet;

fn fulfills_rule1(nums: &[i64]) -> bool {
    let mut possible_sums = BTreeSet::new();
    let mut new_sums = vec![];
    for &num in nums {
        new_sums.extend(possible_sums.iter().map(|n| n + num));
        for new_sum in new_sums.drain(..).chain(std::iter::once(num)) {
            if !possible_sums.insert(new_sum) {
                //println!("lolno: {:?}", nums);
                return false
            }
        }
    }
    true
}

// iterate set of numbers fulfilling rule 2 and check
// whether they fulfill rule 1
fn find_special_sum_subset(total_sum: i64, len: usize) -> Vec<i64> {
    let mut nums = vec![];
    _find_special_sum_subset(total_sum, len, &mut nums);
    nums
}

fn _find_special_sum_subset(residual: i64, residual_len: usize, nums: &mut Vec<i64>) -> bool {
    if residual_len == 0 && residual == 0 {
        return fulfills_rule1(nums)
    }

    // maximum for the new number
    // while still fulfilling rule 2
    // the first l lowest numbers need to be larger than the l-1 last numbers
    // new num is part of the last nums => max_new_num = sum(nums[0..l]) - sum(nums[-(l-2)..])
    let max_new_num = (2..nums.len() / 2 + 1 + 1)
        .map(|l| {
            nums[0..l].iter().sum::<i64>() - nums[nums.len()-(l-2)..].iter().sum::<i64>()
        }).min().unwrap_or(std::i64::MAX) - 1;

    let min = nums.last().cloned().unwrap_or(0) + 1;
    for n in min..std::cmp::min(residual, max_new_num)+1 {
        nums.push(n);
        if _find_special_sum_subset(residual-n, residual_len-1, nums) {
            return true
        };
        nums.pop();
    }
    false
}

fn main() {
    let optimal_sum_subset = (120..).map(|sum| find_special_sum_subset(sum, 7))
        .find(|nums| nums.len() != 0)
        .unwrap();
    for num in &optimal_sum_subset {
        print!("{}", num);
    }
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
