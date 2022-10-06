#![feature(clone_from_slice)]
#![feature(iter_arith)]
#[macro_use] extern crate itertools;
use std::cmp::min;

fn min4<T: Ord>(a: T, b: T, c: T, d: T) -> T { min(min(min(a,b), c), d) }

fn row_sub(mut a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    for (cell, &num_b) in izip!(&mut a, &b) {
        *cell -= num_b;
    }
    a
}

fn count_solutions() -> u64 {
    let mut grid = [0; 16];
    let mut count = 0;
    for first_row in iproduct!(0..10, 0..10, 0..10, 0..10)
        .map(|(a,b,c,d)| [a,b,c,d])
    {
        grid.clone_from_slice(&first_row); // copy first fields in
        let row_sum = (&grid[0..4]).iter().sum();
        let rests = Rests {
            row: row_sum,
            cols: row_sub([row_sum; 4], first_row),
            diagonals: [row_sum - grid[0], row_sum - grid[3]],
        };
        //println!("{}, {}", first_row[0], count);
        count += fill_row(grid, 1, 0, rests, row_sum);
        //println!("{:?}, sum: {}\nRests: {:?}", grid, row_sum, rests);
        //break
    }
    count
}

fn cell_nr(row: usize, col: usize) -> usize { row*4 + col }

#[derive(Clone, Copy, Debug)]
struct Rests {
    row: u8,
    cols: [u8; 4],
    diagonals: [u8; 2],
}

fn limit(rests: Rests, row: usize, col: usize) -> u8 {
    let limit_diagonal = match cell_nr(row, col) {
        0 | 5 | 10 | 15 => rests.diagonals[0],
        3 | 6 | 9  | 12 => rests.diagonals[1],
        _ => 9, // == no limit
    };
    min4(9, rests.row, rests.cols[col], limit_diagonal)
}

fn new_rests(mut rests: Rests, num: u8, row: usize, col: usize) -> Rests {
    rests.row -= num;
    rests.cols[col] -= num;
    match cell_nr(row, col) {
        0 | 5 | 10 | 15 => rests.diagonals[0] -= num,
        3 | 6 | 9  | 12 => rests.diagonals[1] -= num,
        _ => (),
    };
    rests
}

fn fill_row(
    mut grid: [u8; 16],
    row: usize,
    col: usize,
    rests: Rests,
    row_sum: u8
    ) -> u64
{
    let cell_nr = cell_nr(row, col);
    match (row, col) {
        (3, 0) => { // last row, everything is determined
            if rests.cols[0] == rests.diagonals[1]
            && rests.cols[3] == rests.diagonals[0]
            && rests.cols.iter().sum::<u8>() == row_sum
            && rests.cols.iter().all(|&rest| rest < 10) {
                1
            } else {
                0
            }
        },
        (0...2, 3) => { // last in row
            let n = rests.row;
            if n >= 10 { return 0 }
            grid[cell_nr] = n;
            let mut rests = new_rests(rests, n, row, col);
            rests.row = row_sum; // reset row_sum
            fill_row(grid, row+1, 0, rests, row_sum)
        },
        (0...2, 0...2) => {
            (0..limit(rests, row, col) + 1).map(|n| {
                grid[cell_nr] = n;
                let rests = new_rests(rests, n, row, col);
                fill_row(grid, row, col+1, rests, row_sum)
            }).sum()
        },
        _ => unreachable!(),
    }
}

fn main() {
    println!("{}", count_solutions());
}
