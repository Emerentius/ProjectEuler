extern crate num;
use num::integer::gcd;

fn main() {
    let max_n = 1_000_000_000;
    let sqrt_max_n = (max_n as f64).sqrt() as usize;

    let mut sum = 0;
    for a in 1..sqrt_max_n+1 {
        let r_a = a*a;
        for c in 1..a {
            let r_c = c*c;
            let x_c = 2*a*c;
            for b in (a*c..sqrt_max_n+1).step_by(a*c) {
                let r_b = b*b;
                let x_b = 2*a*b;

                if x_b*x_b + x_c*x_c == 4*r_b*r_c + 2*x_b*x_c {
                    if gcd(a,gcd(b,c)) != 1 {
                        continue
                    }
                    let n_sols = max_n / r_b;
                    sum += n_sols*(n_sols+1)/2 * (r_a + r_b + r_c);
                }
            }
        }
    }

    println!("{}", sum);
}
