#![feature(test)]
extern crate test;

use euler_utils::num::pow_mod;

fn main() {
    // num_subsets[rest] modulo 10^16
    let mut num_subsets = [0; 250];
    num_subsets[0] = 1; // empty subset

    // dynamic programming
    // build up counts of subsets while increasing the set up to 250250^250250.
    for num in 1..=250250 {
        let rest = pow_mod(num, num as u32, 250);

        let old_num_subsets = num_subsets;
        for old_rest in 0..250 {
            let new_rest = (rest + old_rest) % 250;
            num_subsets[new_rest as usize] = (num_subsets[new_rest as usize]
                + old_num_subsets[old_rest as usize])
                % 10u64.pow(16);
        }
    }
    // - 1 to get rid of the empty subset
    println!("{}", num_subsets[0] - 1);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(main);
}
