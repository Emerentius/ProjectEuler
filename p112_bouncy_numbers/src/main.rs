#![feature(test)]
extern crate test;
use std::cmp::Ordering::*;

enum Direction {
    Increasing,
    Bouncy,
    Decreasing,
    Equal,
}

fn classify_num (mut n:u64) -> Direction {
    let (mut is_rising, mut is_falling) = (true, true);
    let mut last_digit = n % 10;
    n /= 10;
    while n != 0 {
        let digit = n % 10;

        match digit.cmp(&last_digit) {
            Greater => is_rising  = false,
            Less    => is_falling = false,
            Equal   => {},
        }
        if !is_rising && !is_falling { break }
        last_digit = digit;
        n /= 10;
    }

    match (is_rising, is_falling) {
        (true, true)   => Direction::Equal,
        (true, false)  => Direction::Increasing,
        (false, true)  => Direction::Decreasing,
        (false, false) => Direction::Bouncy,
    }
}

fn main() {
    let mut bouncy = 0;
    let mut non_bouncy = 0;
    for i in 1.. {
        match classify_num(i) {
            Direction::Bouncy => bouncy += 1,
            _                 => non_bouncy += 1,
        }
        if bouncy == 99*non_bouncy {
            println!("{}", i);
            break
        }
    }
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
