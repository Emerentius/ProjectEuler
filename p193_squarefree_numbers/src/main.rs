#![feature(test)]
extern crate test;
extern crate primal; // prime library
extern crate itertools; // for .while_some()
use itertools::Itertools;

const LIMIT: usize = 1125899906842624; // 2^50

// include-exclude principle
fn count_squarefree(squares_product: usize, squares: &[usize]) -> usize {
    let mut sum = LIMIT / squares_product;
    for (i, new_prod) in squares.iter()
        .map(|sq| sq.checked_mul(squares_product)) // overflow guard
        .while_some()
        .take_while(|&new_prod| new_prod <= LIMIT )
        .enumerate()
    {
        sum -= count_squarefree(new_prod, &squares[i+1..]);
    }
    sum
}

fn main() {
    let prime_squares = primal::Primes::all()
        .map(|n| n*n)
        .take_while(|&nn| nn < LIMIT)
        .collect::<Vec<_>>();
    println!("{}", count_squarefree(1, &prime_squares));
}


#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
