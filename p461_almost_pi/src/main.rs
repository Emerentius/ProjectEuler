extern crate ord_subset;
use ord_subset::OrdVar;
use std::f64::consts::PI;
const N: f64 = 10_000.0;

fn f(k: u32) -> f64 { (k as f64/N).exp() - 1.0 }

fn main() {
    // k1^2+k2^2 and f1+f2 for any combination of k1, k2, f1, f2: f1 + f2 < Pi
    let mut f_sums = vec![];

    for (k1, f1) in (0..).map(|k| (k, f(k)))
        .take_while(|&(_,f1)| f1 < PI)
    {
        for (k2, f2) in (k1..).map(|k| (k, f(k)))
            .take_while(|&(_, f2)| f1 + f2 < PI)
        {
            f_sums.push( (f1+f2, k1*k1 + k2*k2) );
        }
    }

    f_sums.sort_by(|&(f1, _), &(f2, _)| f1.partial_cmp(&f2).unwrap());

    let mut min_err = std::f64::INFINITY;
    let mut k_sq_sum = 0;

    for &(f12_sum, k12_sq_sum) in f_sums.iter() {
        let idx = f_sums.binary_search_by(|&(f_sum, _)| {
                f_sum.partial_cmp(&(PI-f12_sum)).unwrap()
            })
            .unwrap_err();
        let &&(f34_sum, k34_sq_sum) = &f_sums[idx-1..].iter()
            .take(3)
            .min_by_key(|&&(f34_sum, _)| OrdVar::new((PI - f12_sum - f34_sum).abs()) )
            .unwrap();
        let err = (PI - f12_sum - f34_sum).abs();
        if err < min_err {
            min_err = err;
            k_sq_sum = k12_sq_sum + k34_sq_sum;
        }
    }
    println!("k_sq_sum: {}\nminimal error: {}", k_sq_sum, min_err);
}
