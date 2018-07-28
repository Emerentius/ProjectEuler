#![feature(step_by)]
extern crate primal;
extern crate crossbeam;

const LIMIT: usize = 10_000_000;

fn is_idempotent(num: usize, mod_: usize) -> bool {
    num * num % mod_ == num
}

fn walk_factorisations(mut n_idem: u16, num: usize, primes: &[usize], n_idempotents: &mut [u16]) {
    n_idem *= 2;
    for (i, &pr) in primes.iter().enumerate() {
        let mut new_num = num * pr;
        if new_num > LIMIT { break }
        while new_num <= LIMIT {
            n_idempotents[new_num-1] = n_idem;
            walk_factorisations(n_idem, new_num, &primes[i+1..], n_idempotents);
            new_num *= pr;
        }
    }
}

fn main() {
    let primes = primal::Primes::all()
        .take_while(|&pr| pr <= LIMIT)
        .collect::<Vec<_>>();

    let mut n_idempotents = vec![0; LIMIT];
    walk_factorisations(1, 1, &primes, &mut n_idempotents);

    crossbeam::scope(|scope| {
        let mut threads = vec![];
        threads.push( scope.spawn(|| {
            let mut sum = 0;
            for n in (4..LIMIT+1).step_by(4) {
                if n % 1_000 == 0 { println!("{}", n) }
                if n_idempotents[n-1] == 2 {
                    sum += 1;
                    continue
                }
                sum += n + 1 - (2..).find(|&a| is_idempotent(a, n)).unwrap()
            }
            sum
        }));

        for i in 1..4 {
            let n_idempotents = &n_idempotents;
            threads.push( scope.spawn(move || {
                let mut sum = 0;
                for n in (4+i..LIMIT+1).step_by(4) {
                    if n_idempotents[n-1] == 2 {
                        sum += 1;
                        continue
                    }
                    sum += n + 1 - (2..).find(|&a| is_idempotent(a, n)).unwrap()
                }
                sum
            }));
        };

        // account for numbers 1, 2, 3 that were left out above
        let mut total_sum = 2;
        for jh in threads {
            total_sum += jh.join();
        }
        println!("{}", total_sum);
    })
}
