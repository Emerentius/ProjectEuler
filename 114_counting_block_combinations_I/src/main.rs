#![feature(test)]
#![feature(unboxed_closures)]
#![feature(core)]
extern crate test;
use std::collections::BTreeMap;
use std::ops::{FnOnce, FnMut};

const MIN_BLOCK_LEN: u64 = 3;
const ROW_LEN: u64 = 50;
const ROW_LEN_PLUS: u64 = 51;

struct CombinationCounter {
    memoized_values: BTreeMap<u64, u64>,
}

impl FnOnce<(u64,)> for CombinationCounter {
    type Output = u64;
    extern "rust-call" fn call_once(mut self, (pos,): (u64,)) -> Self::Output
    {
        (self)(pos)
    }
}

impl FnMut<(u64,)> for CombinationCounter {
    extern "rust-call" fn call_mut(&mut self, (pos,): (u64,)) -> Self::Output
    {
        match pos {
            ROW_LEN_PLUS => 1,
            0...ROW_LEN => {
                if let Some(&val) = self.memoized_values.get(&pos) {
                    return val;
                }

                let mut sum = 0;

                // empty block
                sum += (self)(pos + 1);
                for block_len in (MIN_BLOCK_LEN..ROW_LEN-pos+1) {
                    sum += (self)(pos + block_len + 1);
                }

                self.memoized_values.insert(pos, sum);
                sum
            }
            _ => 0,
        }
    }
}

// pos starting at 0
fn count_block_combinations(pos: u64) -> u64 {
    match pos {
        ROW_LEN_PLUS => 1,
        0...ROW_LEN => {
            let mut sum = 0;

            // empty block
            sum += count_block_combinations(pos + 1);
            for block_len in (MIN_BLOCK_LEN..ROW_LEN-pos+1) {
                sum += count_block_combinations(pos + block_len + 1);
            }
            sum
        }
        _ => 0,
    }
}

fn main() {
    let mut counter = CombinationCounter {
        memoized_values: BTreeMap::new()
    };
    println!("{}", counter(0));
    //println!("{}", count_block_combinations(0));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
