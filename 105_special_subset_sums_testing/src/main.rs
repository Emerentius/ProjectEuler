#![feature(test)]
extern crate test;

use std::collections::BTreeSet;

fn fulfills_rule1(nums: &[u64]) -> bool {
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

// assumes sorted input
fn fulfills_rule2(nums: &[u64]) -> bool {
    let l = (nums.len()+1)/2;
    nums.iter().take(l).sum::<u64>() > nums.iter().rev().take(l-1).sum::<u64>()
}

fn main() {
    let sets = include_str!("p105_sets.txt")
        .lines()
        .map(|line| {
            let mut set = line.split(',')
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            set.sort();
            set
        })
        .collect::<Vec<_>>();

    let total_sum = sets.iter()
        .filter(|set| fulfills_rule2(&set[..]))
        .filter(|set| fulfills_rule1(&set[..]))
        .map(|set| set.iter().sum::<u64>())
        .sum::<u64>();
    println!("{}", total_sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
