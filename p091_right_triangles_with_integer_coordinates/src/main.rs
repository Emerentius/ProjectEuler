#![feature(test)]
extern crate test;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vector(i32, i32);

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Self) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}

// scalar product
impl Mul for Vector {
    type Output = i32;
    fn mul(self, other: Self) -> i32 {
        self.0 * other.0 + self.1 * other.1
    }
}

fn main() {
    let max_size = 50 + 1;
    let p0 = Vector(0,0);

    let mut count = 0;
    for x1 in 0..max_size {
        for y1 in 0..max_size {

            let p1 = Vector(x1, y1);
            if p1 == p0 { continue }

            'x2: for x2 in 0..max_size {
                for y2 in 0..max_size {
                    if x2 <= x1 && y2 >= y1 { continue 'x2 }

                    let p2 = Vector(x2, y2);
                    if p2 == p0 || p2 == p1 { continue }

                    if p1*p2 == 0
                    || (p2-p1)*p1 == 0
                    || (p2-p1)*p2 == 0 {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
