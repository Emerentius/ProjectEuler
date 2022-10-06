#![feature(test)]
extern crate test;

const LIMIT: u64 = 100_000_000_000_000_000; // 10^17

fn main() {
    let mut fib = vec![1,2]; // skip terms not in zeckendorf-repr.
    let mut next_fib = 3;
    while next_fib < LIMIT {
        fib.push(next_fib);
        next_fib = fib[ fib.len()-2 ] + fib[ fib.len() - 1 ];
    }

    let mut sum_z = vec![0; fib.len()];
    sum_z[1] = 1;
    for i in 2..fib.len() {
        sum_z[i] = sum_z[i-2] + sum_z[i-1] + fib[i-2]; // not fib[i-3] because of skip before
    }

    // sum_z.last() is sum(z(n)) up to last_fib-1
    // now add rest of z(n), n in (last_fib..LIMIT)
    let mut sum = *sum_z.last().unwrap();
    let mut diff = LIMIT - fib.last().unwrap();

    while diff != 0 {
        let (&f, &s) = Iterator::zip(fib.iter(), sum_z.iter() ).rev()
            .find(|&(&fib, _)| diff >= fib)
            .unwrap();
        sum += s + diff;
        diff -= f;
    }
    println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| {
        test::black_box({
            let mut fib = vec![1,2]; // skip terms not in zeckendorf-repr.
            let mut next_fib = 3;
            while next_fib < LIMIT {
                fib.push(next_fib);
                next_fib = fib[ fib.len()-2 ] + fib[ fib.len() - 1 ];
            }

            let mut sum_z = vec![0; fib.len()];
            sum_z[1] = 1;
            for i in 2..fib.len() {
                sum_z[i] = sum_z[i-2] + sum_z[i-1] + fib[i-2]; // not fib[i-3] because of skip before
            }

            // sum_z.last() is sum(z(n)) up to last_fib-1
            // now add rest of z(n), n in (last_fib..LIMIT)
            let mut sum = *sum_z.last().unwrap();
            let mut diff = LIMIT - fib.last().unwrap();

            while diff != 0 {
                let (&f, &s) = Iterator::zip(fib.iter(), sum_z.iter() ).rev()
                    .find(|&(&fib, _)| diff >= fib)
                    .unwrap();
                sum += s + diff;
                diff -= f;
            }
            sum
        });
    })
}

#[bench]
fn nop(b: &mut test::Bencher) {
    b.iter(|| ())
}
