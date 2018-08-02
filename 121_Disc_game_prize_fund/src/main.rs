#![feature(test)]
extern crate test;

use std::ops::Add;
const MAX_TURNS: u64 = 15;
const WIN: u64 = 8; // MAX_TURNS / 2 + 1

#[derive(Debug)]
struct WinsLosses(u64, u64);

impl Add<WinsLosses> for WinsLosses {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        WinsLosses(self.0 + rhs.0 , self.1 + rhs.1)
    }
}

fn probability(n_turns: u64, possibilities: u64, n_blue: u64) -> WinsLosses {
    match n_turns {
        MAX_TURNS => {
            match n_blue {
                WIN...MAX_TURNS => WinsLosses(possibilities, 0),
                _               => WinsLosses(0, possibilities),
            }
        },
        _ => probability(n_turns+1, (n_turns+1)*possibilities, n_blue)
           + probability(n_turns+1, possibilities, n_blue+1),
    }
}

fn main() {
    let WinsLosses(wins, losses) = probability(0,1,0);
    println!("{:?}", losses/wins + 1);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
