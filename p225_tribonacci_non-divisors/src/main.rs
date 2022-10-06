#![feature(test)]
extern crate test;
use std::collections::HashSet;

fn is_tribonacci_divisor(num: u64) -> bool {
    let mut seen_before = HashSet::new();
    let mut last_nums = [1, 1, 1];
    while !seen_before.contains(&last_nums) {
        let next_num = last_nums.iter().sum::<u64>() % num;
        if next_num == 0 { return true }
        seen_before.insert(last_nums);
        last_nums = [last_nums[1], last_nums[2], next_num];
    }
    false
}

fn main() {
    let solution = (3..).step_by(2)
        .filter(|&num| !is_tribonacci_divisor(num))
        .take(124)
        .last()
        .unwrap();
    println!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
