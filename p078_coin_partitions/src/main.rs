#![feature(test)]
extern crate test;

fn pent_nr(n: i64) -> i64 { n*(3*n-1)/2 }

fn main() {
    // only interested in partition func % 10^6
    // recurrence formula is a sum => only storing p % 10^6 works
    let mut p_func = vec![1, 1];

    for n in 2.. {
        // compute next partition function recursively
        let mut next_p = 0;
        'k: for k in 1.. {
            for pent in [k, -k].iter().cloned().map(pent_nr) {
                if pent > n { break 'k }
                match k % 2 == 0 {
                    true =>  next_p -= p_func[(n - pent) as usize],
                    false => next_p += p_func[(n - pent) as usize],
                };
                next_p %= 1_000_000;
            }
        }

        if next_p == 0 {
            println!("{}", n);
            break
        } else {
            p_func.push(next_p);
        }
    }
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
