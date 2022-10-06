use num::Integer;

fn eulercoins() -> impl Iterator<Item = i128> {
    let a = 1504170715041707;
    let m = 4503599627370517;

    let ext_gcd = a.extended_gcd(&m);
    let a_inv = ext_gcd.x;

    let mut min_ = a;
    let mut x = a;

    std::iter::once(min_).chain(std::iter::from_fn(move || {
        if min_ > 10i128.pow(8) {
            // naive bruteforce
            loop {
                x = (x + a) % m;
                if x < min_ {
                    min_ = x;
                    return Some(min_);
                }
            }
        } else {
            // check for all numbers below the last eulercoin what `n*a` we need to reach it.
            //    x + n*a = y   for all y < x
            // => n*a = y - x
            //    n = (y - x) * a^-1   (mod m)
            // the minimium `n` will be the next eulercoin.
            let n_to_next_eulercoin = (1..min_)
                .map(|target| n_for(target, min_, a_inv, m))
                .min()?;
            x = (x + n_to_next_eulercoin * a).rem_euclid(m);
            min_ = x;
            return Some(min_);
        }
    }))
}

fn n_for(target: i128, current: i128, a_inverse: i128, modulo: i128) -> i128 {
    ((target - current) * a_inverse).rem_euclid(modulo)
}

fn main() {
    println!(
        "sum: {}",
        eulercoins()
            .inspect(|coin| println!("{}", coin))
            .sum::<i128>()
    );
}
