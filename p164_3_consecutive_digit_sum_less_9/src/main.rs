#![feature(test)]
extern crate test;

use std::collections::BTreeMap;
fn main() {
    // BTreeMap[(2digit_sum, remaining_length)] -> n_poss
    let mut length = 3;
    // n_poss[remaining_length] = Map<[first, middle]> -> count
    let mut n_poss: Vec<BTreeMap<_, _>> = vec![];

    // Generate iterator over i,j i+j <= 9
    let j_iter = |i| (0..9+1-i).map(move |j| (i, j));
    let ij_iter = || (0..9+1).flat_map(|i| j_iter(i));

    // enter possibilities counts for all 2-long subsequences for nums of total length 3
    n_poss.push(
        ij_iter().map(|(i, j)| ([i, j], 10-i-j)).collect()
    );

    // recursively add possibilities counts for longer numbers
    while length != 20 {
        let mut n_poss_next = BTreeMap::new();
        {
            let n_poss_last = &n_poss[length-3];
            for (i, j) in ij_iter() {
                let n_poss: u64 = (0..9+1-i-j).map(|k| n_poss_last[&[j, k]]).sum();
                n_poss_next.insert([i, j], n_poss);
            }
        }
        n_poss.push(n_poss_next);
        length += 1;
    }

    let solution: u64 = n_poss.last().unwrap()
        .iter()
        .filter(|&(subseq, _)| subseq[0] != 0)
        .map(|(_, num)| num)
        .sum();
    print!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
