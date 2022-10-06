extern crate primal;
use primal::is_prime;

fn main() {
    let mut sum = 0;
    let summands = [1u64,3,7,9,13,27];
    let forb_summands = [15, 19, 21];
    'n: for n in (10..150_000_000u64).step_by(10) {
        if n % 3 == 0 || n % 7 == 0
        || n % 13 == 0 {
            continue
        }
        let n_sq = n*n;
        if summands.iter().all(|&s| is_prime(n_sq + s) )
        && !forb_summands.iter().any(|&fs| is_prime(n_sq +fs)) {
            sum += n;
            println!("{}: {}", n, n_sq);
        }
    }
    println!("{}", sum);
}
