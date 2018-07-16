#![feature(test)]
extern crate test;
extern crate num;
use num::integer::gcd;
use std::collections::BTreeSet;

fn points_inside (h:usize, w: usize) -> usize {
    // pick's theorem
    let boundary_points = gcd(h,w) + h + w;
    (h*w+ 2 - boundary_points)/2
}

fn main() {
    const m : usize = 100;

    let squares: BTreeSet<_> = (1..142).map(|n| n*n).collect();
    let is_square = |num| {
        match num % 16 {
            0 | 1 | 4 | 9 => squares.contains(&num),
            _ => false,
        }
    };

    let mut points = [[0;101]; 101];
    for (i, inner) in points.iter_mut().enumerate() {
        for (j, entry) in inner.iter_mut().enumerate() {
            *entry = points_inside(i,j);
        }
    }

    let mut counter = 0;
    for A in 1..m+1 {
        for C in A..m+1 {
            for B in 1..m+1 {
                let p2 = points[A][B] + points[C][B];
                for D in B..m+1 {
                    let p4 = A+B+C+D-4+1+ p2 + points[A][D] + points[C][D];
                    if is_square(p4) {
                        counter += match (A == C, B == D) {
                            (true, true) => 1,
                            (true, false) | (false, true) => 2,
                            (false, false) => 4,
                        };
                    }
                }
            }
        }
    }
    println!("{}", counter);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
