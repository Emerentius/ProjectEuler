use std::cmp::Ordering::*;

fn _min_prod_sum_num(k: u64, n_nums_remaining: u64, last_num: u64, sum: u64, prod: u64) -> u64 {
    match (sum + n_nums_remaining).cmp(&prod) {
        Equal => prod,
        Less => !0, // dead end
        Greater => {
            (last_num..2*k)
                .take_while(|&num| prod * num <= 2*k)
                .map(|num| _min_prod_sum_num(k, n_nums_remaining-1, num, sum + num, prod * num))
                .min().unwrap_or(!0)
        }
    }
}

fn min_prod_sum_num(k: u64) -> u64 {
    (2..2*k).map(|num| _min_prod_sum_num(k, k-1, num, num, num)).min().unwrap()
}

fn main() {
    let mut min_nums = (2..12000+1)
        .map(min_prod_sum_num)
        .collect::<Vec<_>>();
    min_nums.sort();
    min_nums.dedup();
    println!("{}", min_nums.iter().sum::<u64>());
}
