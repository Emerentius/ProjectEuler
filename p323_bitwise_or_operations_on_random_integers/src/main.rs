#![feature(test)]
extern crate test;

fn binom_coeff(n: u64, k: u64) -> u64 {
    // The real deal. Recursive => no bigints necessary
    fn binom_coeff_helper(n: u64, k: u64) -> u64 {
        match k {
            0 => 1,
            _ => n*binom_coeff(n-1, k-1)/k,
        }
    }

    // for optimisation only
    match k > n/2 {
        true => binom_coeff_helper(n, n-k),
        false => binom_coeff_helper(n,k),
    }
}

fn main() {
    let mut expected_val = [0_f64; 33];

    for n in 1..32+1 {
        let mut sum = 0.;
        let mut normaliser = 0.;
        for (i, &exp_val) in expected_val.iter().take(n).enumerate() {
            let binom = binom_coeff(n as u64,i as u64) as f64;
            sum += (exp_val+1.) * binom;
            normaliser += binom;
        }
        expected_val[n] = (sum + 1.) / normaliser;
    }
    print!("{:.10}", expected_val[32]);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
