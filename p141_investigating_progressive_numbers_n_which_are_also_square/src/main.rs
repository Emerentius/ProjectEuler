use num::Integer;

fn main() {
    let mut sum = 0;
    let limit = 1_000_000_000_000;
    // When looking at divisors less than √n
    // rest < divisor < quotient
    //
    // r = r          = k b² (as r must be divisible by b² for q to be an integer)
    // d = r a/b    = k a b
    // q = r a²/b²  = k a²
    //
    // n = r + d q
    //   = k b² + (k a b) (k a²)
    //   = k b (b + k a³)
    // => a^3 <= limit
    for a in 2..10_000u64 {
        // b < a, so ratio is > 1
        for b in 1..a {
            // optimization only
            if a * a * a * b >= limit {
                break;
            }

            if a.gcd(&b) != 1 {
                continue;
            }

            for k in 1.. {
                let n = k * b * (b + a * a * a * k);
                if n >= limit {
                    break;
                }
                let quotient = k * a * a;
                let rest = b * b * k;
                let divisor = k * a * b;

                debug_assert!(divisor > rest);
                debug_assert!(quotient > divisor);

                let sqrt = (n as f64).sqrt() as u64;
                if sqrt * sqrt == n {
                    println!(
                        "{n} √n={sqrt} a/b={} rest={rest} d={divisor} q={quotient} a={a} b={b} k={k}",
                        num::rational::Ratio::new(divisor, rest)
                    );
                    sum += n;
                }
            }
        }
    }
    println!("{sum}");
}
