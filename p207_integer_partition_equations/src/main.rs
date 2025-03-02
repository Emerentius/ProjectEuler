fn main() {
    // 4^t = 2^t + k
    // => 2^t(2^t - 1) = k
    // all numbers are integer if 2^t is an integer
    // 2^t = n
    // t is solution for any integer n (>=2) and it's an integer solution if n is a power of 2.
    //
    // For a given k, the upper boundary for n is
    // 2^t(2^t - 1) = k
    // = n(n-1) = k
    // => n² - n - k = 0
    // => max_n = 0.5 ± √(0.25 + k), positive branch to get a real solution
    //
    // Fraction of solutions is:
    // floor(log2(max_n)) / (floor(max_n) - 1)
    // = floor(log2(0.5 + sqrt(0.25 + k))) / (floor(0.5 + sqrt(0.25 + k)) - 1)
    // ≅ log2(sqrt(k)) / sqrt(k)
    // = log2(k) / (2*sqrt(k))
    // => k ≅ 47_964_500_000 (or 1.0001... which we can ignore)
    // Somewhere in the vicinity of this is the real solution.
    //
    // It's probably possibly to do it by hand if you compute m from n.
    // The proportion is very easy to compute in terms of n and the powers of 2 also trivial to get.
    //
    // Inefficient brute force starting from some point below it.
    for m in 47_964_500_000u64 - 4_000_000_000.. {
        let max_n = 0.5 + (0.25 + m as f64).sqrt();
        let proportion = max_n.log2().floor() / (max_n.floor() - 1.0);
        if proportion < 1.0 / 12345. {
            let numeratior = max_n.log2().floor() as u64;
            let denominator = max_n.floor() as u64 - 1;
            println!("{m}: {numeratior}/{denominator} = {proportion}");
            break;
        }
    }
}
