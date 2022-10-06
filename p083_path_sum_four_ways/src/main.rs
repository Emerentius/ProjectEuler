#![feature(test)]
extern crate test;
use std::collections::BinaryHeap;

const N: usize = 79;

use Cell::*;
#[derive(Copy, Clone)]
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

fn main() {
    // read file at compile time
    let matrix_string = include_str!("p083_matrix.txt");
    // parse string into matrix
    // matrix[row][col]
    let mut matrix : Vec<Vec<Cell>> = matrix_string.lines()
            .map(|line|
                line.split(',')
                    .map(|s| NotVisited(s.parse().unwrap()) )
                    .collect()
            ).collect();
    matrix[0][0] = Start( matrix[0][0].val() );
    matrix[N][N] = Finish( matrix[N][N].val() );

    // priority queue
    // (value, row, col)
    let mut queue = BinaryHeap::new(); // max heap

    let mut cur_row = 0;
    let mut cur_col = 0;

    'outer: loop { //'
        let cur_val = matrix[cur_row][cur_col].val();

        // loop through  existing neighbours
        // save path sum into new cells and push them onto queue
        for &(row_off, col_off) in &[(0isize ,1isize), (0,-1), (1,0), (-1,0)] {
            // n_row/col = neighbour_row/col
            let (n_row, n_col) = (cur_row + row_off as usize, cur_col + col_off as usize);
            let neighb = matrix.get(n_row)
                .and_then(|vec| vec.get(n_col))
                .cloned();
            if let Some(entry) = neighb {
                match entry {
                    // Uninteresting paths, can only increase path sum needlessly
                    Visited(_) | Start(_) => (),
                    NotVisited(a) => {
                        // max heap => invert so min floats up
                        queue.push( (!(cur_val+a), n_row, n_col) );
                        matrix[n_row][n_col] = Visited(cur_val+a);
                    },
                    // Optimal path found. Compute sum.
                    Finish(a) => {
                        println!("path sum: {}", a+cur_val);
                        break 'outer // '
                    }
                };
            }
        }
        //neighbours.clear();

        // move to next Cell in queue
        match queue.pop() {
            Some( (_, row, col) ) => {
                cur_row = row;
                cur_col = col;
            },
            None => unreachable!(),
        };
    }
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
