#![feature(test)]
extern crate test;
use std::cmp::min;

fn main() {
    let matrix_string = include_str!("p081_matrix.txt");
    // matrix[row][column]
    let mut matrix : Vec<Vec<u64>> = matrix_string.lines()
            .map(|line|
                line.split(',')
                    .map(|s| s.parse().unwrap() )
                    .collect()
            ).collect();

    for row in (0..80).rev() {
        for col in (0..80).rev() {
            matrix[row][col] += match (row, col) {
                (79,79) => 0,
                (79,_) => matrix[row][col+1],
                (_,79) => matrix[row+1][col],
                (_,_) => min(matrix[row+1][col], matrix[row][col+1]),
            };
        }
    }

    println!("{}", matrix[0][0]);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
