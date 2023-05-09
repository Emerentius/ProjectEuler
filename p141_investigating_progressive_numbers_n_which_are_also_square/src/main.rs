fn main() {
    let mut sum = 0;
    for n_root in 2..1_000_000u64 {
        let n = n_root * n_root;
        // testing divisors greater than √n is pointless as we would get the same
        // tuple of (n,d,q), just in a different order.
        //
        // divisor < quotient
        // rest < divisor
        // rest < quotient
        // => rest < divisor < quotient
        for divisor in 2..n_root {
            let quotient = n / divisor;
            let rest = n % divisor;
            if rest == 0 {
                continue;
            }

            debug_assert!(divisor > rest);
            debug_assert!(quotient > divisor);
            // a_(k+1) = a_k * r
            // r = a_(1+1) / a_1 = a_(1+2) / a_(1+1)
            // <=> (a_2)^2 = a_1 * a_3
            if rest * quotient == divisor * divisor {
                println!(
                    "{n} √n={n_root} r={} d={divisor} q={quotient}",
                    num::rational::Ratio::new(divisor, rest)
                );
                sum += n;
                break;
            }
        }
    }
    println!("{sum}");
}
