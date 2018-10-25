#![feature(test)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
extern crate test;
use std::collections::BTreeMap;
use std::ops::{FnOnce, FnMut};

const MIN_BLOCK_LEN: u64 = 50;

struct CombinationCounter {
    memoized_values: BTreeMap<u64, u64>,
    row_len: u64,
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
        if pos == self.row_len + 1 {
            1
        } else if pos > self.row_len + 1 {
            0
        } else {
            if let Some(&val) = self.memoized_values.get(&pos) {
                return val;
            }

            let mut sum = 0;

            // empty block
            sum += (self)(pos + 1);
            for block_len in MIN_BLOCK_LEN..self.row_len-pos+1 {
                sum += (self)(pos + block_len + 1);
            }

            self.memoized_values.insert(pos, sum);
            sum
        }
    }
}

fn main() {
    for n in 50.. {
        let mut count_block_combinations = CombinationCounter {
            memoized_values: BTreeMap::new(),
            row_len: n,
        };
        if count_block_combinations(0) > 1_000_000 {
            print!("{}", n);
            break
        }
    }
    //print!("{}", count_block_combinations(0));
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
