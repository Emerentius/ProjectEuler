#![feature(test)]
extern crate test;
extern crate int_sqrt; // integer squareroot
use int_sqrt::IntSqrt;

use std::collections::BTreeSet;

fn main() {
    // [row][col]
    let mut triangle = [[0u64; 51]; 51];
    for row in 0..51 {
        triangle[row][0] = 1;
        triangle[row][row] = 1;
    }

    for row in 1..51 {
        for col in 1..row {
            triangle[row][col] = triangle[row-1][col-1] + triangle[row-1][col]
        }
    }

    let (nums, max) = triangle.iter()
        .flat_map(|row| row.iter().take_while(|&&num| num != 0))
        .cloned()
        .fold( (BTreeSet::new(), 0), |(mut set, mut max), num| {
            if num > max { max = num }
            set.insert(num);
            (set, max)
        });

    let squares: Vec<u64> = (2..max.isqrt()+1).map(|n| n*n).collect();

    let solution = nums.into_iter()
        .filter(|num|
            squares.iter()
                .take_while(|&sq| sq <= num)
                .all(|&sq| num % sq != 0))
        .fold(0, |sum, squarefree_num| sum + squarefree_num);

    println!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
