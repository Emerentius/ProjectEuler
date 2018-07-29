#![feature(test)]
extern crate test;

fn len(mut num : f64, base : f64) -> u64 {
    let mut length = 0;
    while num >= 1. {
        num /= base;
        length += 1;
    }
    length
}

fn main() {
    let base : u64 = 10;
    let mut count = 0;
    for n in 1..base {
        for exp in 1u64.. {
            let num = (n as f64).powi(exp as i32) as f64;
            let length = len(num, base as f64);
            if length == exp {
                count += 1;
                //println!("{}^{} = {}", n,exp, num);
            }
            if length != exp { break }
        }
    }
    println!("{}", count)
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
