#![feature(test)]
extern crate test;
extern crate union_find; // v0.3.1
use union_find::{QuickUnionUf as Uf, UnionFind, Union, UnionResult};
use std::collections::VecDeque;

// some boilerplate so library is usable
#[derive(Clone)]
struct SetSize(u32);

impl Union for SetSize {
    fn union(mut lval: Self, rval: Self) -> UnionResult<Self> {
        lval.0 += rval.0;
        UnionResult::Left(lval)
    }
}

const PM_NUMBER: usize = 524287;

fn main() {
    let mut network: Uf<_> = std::iter::repeat( SetSize(1) ).take(1_000_000).collect();

    // initialize data for lagged fibonacci generator
    let s = (1..55+1)
        .map(|k| (100_003i64 - 200_003*k + 300_007*k*k*k) % 1_000_000)
        .collect::<VecDeque<_>>();
    // create iterator for lfg
    let mut s_iter = (0..).scan(s, |s, _| {
        let next_s = (s[56-24-1]+s[56-55-1]) % 1_000_000;
        s.push_back(next_s);
        s.pop_front()
    });

    let mut n_successful_calls = 0;
    while network.get(PM_NUMBER).0 < 990_000 {
        let (caller, callee) = (s_iter.next().unwrap(), s_iter.next().unwrap());
        if caller != callee {
            n_successful_calls += 1;
            network.union(caller as usize, callee as usize);
        }
    }
    println!("{}", n_successful_calls);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
