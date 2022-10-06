#![feature(test)]
extern crate test;
use std::collections::BinaryHeap;

const N: usize = 79;

use Cell::*;
#[derive(Copy, Clone, Debug)]
enum Cell {
    Start(u64),      // cell value
    NotVisited(u64), // cell value
    Finish(u64),     // cell value
    Visited(u64),    // path_sum so far
}

impl Cell {
    fn val(&self) -> u64 {
        match self {
            &Start(a) | &NotVisited(a) | &Visited(a) | &Finish(a) => a,
        }
    }
}

fn minimum_path(mut matrix: Vec<Vec<Cell>>, start_row: usize) -> u64 {
        // priority queue
        // (value, row, col)
        let mut queue = BinaryHeap::new(); // max heap

        let mut cur_row = start_row;
        let mut cur_col = 0;
        let mut neighbours = vec![];

        loop {
            let cur_val = matrix[cur_row][cur_col].val();

            // determine neighbours
            match cur_row {
                0  => neighbours.push( (cur_row+1, cur_col) ), // Down
                N => neighbours.push( (cur_row-1, cur_col) ),  // Up
                _  => {
                    neighbours.push( (cur_row+1, cur_col) );   // Down+Up
                    neighbours.push( (cur_row-1, cur_col) );
                },
            }

            match cur_col {
                //0  => neighbours.push( (cur_row, cur_col+1) ), // Right
                N => (), //neighbours.push( (cur_row, cur_col-1) ),  // Left
                _  => {
                    neighbours.push( (cur_row, cur_col+1) );   // Right+Left
                    //neighbours.push( (cur_row, cur_col-1) );
                },
            }

            // loop through neighbours, save path sum into new cells and push them onto queue
            // n_row/col = neighbour_row/col
            for &(n_row, n_col) in &neighbours {
                match matrix[n_row][n_col] {
                    // Uninteresting paths, can only increase path sum needlessly
                    Visited(_) | Start(_) => (),
                    NotVisited(a) => {
                        // max heap => invert so min floats up
                        queue.push( (!(cur_val+a), n_row, n_col) );
                        matrix[n_row][n_col] = Visited(cur_val+a);
                    },
                    // Optimal path found. Compute sum.
                    Finish(a) => {
                        queue.push( (!(cur_val+a), n_row, n_col) );
                        //return a+cur_val
                    }
                };
            }
            neighbours.clear();

            // move to next Cell in queue
            match queue.pop() {
                Some( (v, row, col) ) => {
                    if let Finish(a) = matrix[row][col] {
                        return a+matrix[row][col-1].val();
                    }
                    cur_row = row;
                    cur_col = col;
                },
                None => unreachable!(),
            };
        }
}

fn main() {
    // read file at compile time
    let matrix_string = include_str!("p082_matrix.txt");
    // parse string into matrix
    // matrix[row][col]
    let mut matrix : Vec<Vec<Cell>> = matrix_string.lines()
            .map(|line|
                line.split(',')
                    .map(|s| NotVisited(s.parse().unwrap()) )
                    .collect()
            ).collect();
    for i in 0..N {
        matrix[i][0] = Start( matrix[i][0].val() );
        matrix[i][N] = Finish( matrix[i][N].val() );
    }

    println!("{}",
        (0..N).map(|row| minimum_path(matrix.clone(), row)).min().unwrap()
    );
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
