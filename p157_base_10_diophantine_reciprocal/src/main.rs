use core::panic;
use std::cmp::Ordering::{Equal, Greater, Less};

// may fail on a 32bit machine due to abuse of usize
fn main() {
    let primes = primal::Sieve::new(100_000);
    let mut total_solution = 0u128;
    for n in 1..=1i16 {
        for x in -n..=n {
            let min_y = (-x as f64 * 2.0f64.log(5.0)).floor() as i16;
            //let min_y = n;
            for y in std::cmp::max(min_y, -n)..=n {
                let min_x = (-y as f64 * 5.0f64.log(2.0)).floor() as i16;
                // if x < min_x {
                //     continue;
                // }
                if 2.0f64.powi(x.into()) * 5.0f64.powi(y.into()) < 1.0 {
                    continue;
                }

                // if !(0..=n).contains(&(n - x - u)) || !(0..=n).contains(&(n - y - v)) {
                //     continue;
                // }
                let factor1 = 2usize.pow((-x).max(0) as u32) * 5usize.pow((-y).max(0) as u32)
                    + 2usize.pow(x.max(0) as u32) * 5usize.pow(y.max(0) as u32);

                // let (occ_corr_2, occ_corr_5) = if x == 0 && y == 0 {
                //     (1, 0)
                // } else if x < 0 {
                //     (x, 0)
                // } else if y < 0 {
                //     (0, y)
                // } else {
                //     (0, 0)
                // };
                let min_u = (-x).max(0);
                let min_v = (-y).max(0);
                assert!(min_u + x >= 0);
                assert!(min_v + y >= 0);
                let occ_corr_2 = x.min(0);
                let occ_corr_5 = y.min(0);
                // let (occ_corr_2, occ_corr_5) = match (x.cmp(&0), y.cmp(&0)) {
                //     (Less, Less) => unreachable!(),
                //     (Less, Equal) => unreachable!(),
                //     (Less, Greater) => (x, 0),
                //     (Equal, Less) => unreachable!(),
                //     (Equal, Equal) => (1, 0),
                //     (Equal, Greater) => (0, 0),
                //     (Greater, Less) => (0, y),
                //     (Greater, Equal) => (0, 0),
                //     (Greater, Greater) => (0, 0),
                // };
                // println!("factor = {factor1}");
                let prime_factors = primes.factor(factor1).unwrap();
                let n_occ_2 = prime_factors
                    .iter()
                    .find(|(pr, _)| *pr == 2)
                    .map_or(0, |&(_, occ)| occ);
                let n_occ_5 = prime_factors
                    .iter()
                    .find(|(pr, _)| *pr == 5)
                    .map_or(0, |&(_, occ)| occ);
                let n_occ_two_and_five = [
                    (
                        2,
                        match usize::try_from(n + n_occ_2 as i16 + occ_corr_2 - min_u) {
                            Ok(num) => num,
                            Err(_) => continue,
                        },
                    ),
                    (
                        5,
                        match usize::try_from(n + n_occ_5 as i16 + occ_corr_5 - min_v) {
                            Ok(num) => num,
                            Err(_) => continue,
                        },
                    ),
                ];

                let n_combinations: u128 = prime_factors
                    .into_iter()
                    .filter(|&(pr, _)| pr != 2 && pr != 5)
                    .chain(n_occ_two_and_five)
                    .inspect(|&(pr, occ)| println!("\t{pr}^{occ}"))
                    .map(|(_, occurence)| (occurence + 1) as u128)
                    .product();
                println!("factor1={factor1} n={n}, x={x}, y={y}, n_sols={n_combinations}");

                total_solution += n_combinations as u128;

                // for u in 0..=n {
                //     for v in 0..=n {
                //         println!("n={n}, u={u}, v={v}, x={x}, y={y}, n_sols={n_combinations}");
                //         //total_solution +=
                //         //  n_combinations as u128 * (n - x + 1) as u128 + (n - y + 1) as u128;
                //         total_solution += n_combinations as u128;
                //     }
                // }
            }
        }
    }
    println!("{total_solution}");
}
