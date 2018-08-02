#![feature(test)]
extern crate test;

fn prob (num: u8, denom: u8) -> f64 {
    num as f64 / (num+denom) as f64
}

fn exp_num (n: u8, picked: u8, new: u8) -> f64 {
    let mut expected = 0.;

    let prob_new = prob(new, picked);
    let prob_old = prob(picked, new);

    if n == 20 { return prob_new+prob_old }

    // conditionals to prevent integer overflow
    if prob_new != 0. {
        expected += prob_new * (1. + exp_num(n+1, picked + 9, new - 10));
    }
    if prob_old != 0. {
        expected += prob_old * exp_num(n+1, picked - 1, new);
    }

    expected
}

fn main() {
    println!("{:.9}", exp_num(1, 9, 60) );
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
