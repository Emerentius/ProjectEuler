#![feature(test)]
extern crate test;
use std::collections::HashSet;

fn main() {
    let limit : u64 = 1_000_000_000_000;

    //let mut strong_repunits = HashSet::new();
    //let mut sum : u64 = 1;
    let mut strong_repunits = Vec::new();

    for base in 2..limit-1 {
        let mut num = base*(base+1) + 1;
        if num > limit { break }
        while num < limit {
            //if strong_repunits.insert(num) { sum += num }
            strong_repunits.push(num);
            num = num*base + 1;
        }
    }
    strong_repunits.sort();
    strong_repunits.dedup();
    let sum = strong_repunits.iter().fold(1, |sum, &el| sum + el);

    println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
