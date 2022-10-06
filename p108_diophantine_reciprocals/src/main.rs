fn main() {
    for n in 1u64.. {
        let n_solutions = (n+1..2*n+1)
            .filter(|x| n*x % (x-n) == 0)
            .count();

        if n_solutions > 1000 {
            println!("{}: {}", n, n_solutions);
            break
        }
    }
}
