#![feature(test)]
extern crate test;

fn pent_nr(n: i64) -> i64 { n*(3*n-1)/2 }

fn main() {
    // 0 and 1, each 1 way to write them
    let mut p_func = vec![1, 1];

    for n in 2..100+1 {
        // compute next partition function recursively
        let mut next_p = 0;
        'k: for k in 1.. {
            for pent in [k, -k].iter().cloned().map(pent_nr) {
                if pent > n { break 'k }
                match k % 2 == 0 {
                    true =>  next_p -= p_func[(n - pent) as usize],
                    false => next_p += p_func[(n - pent) as usize],
                };
            }
        }

        p_func.push(next_p);
    }
    // the above also counts n = n + 0 as a valid way
    // therefore subtract 1
    print!("{}", p_func[100]-1);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
