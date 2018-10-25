fn main() {
    const MAX: usize = 40_000_000;
    const CHAIN_LEN: usize = 25;

    let mut sum = 0;
    let mut totients: Vec<_> = (0..MAX).collect();
    let mut chain_len = vec![0; MAX];
    chain_len[1] = 1; // offset of 1, applies to all chain lengths thereafter

    for i in 2..MAX {
        if totients[i] == i { // => i prime
            for n in (i..MAX).step_by(i) {
                totients[n] -= totients[n] / i;
            }
            if chain_len[totients[i]] + 1 == CHAIN_LEN {
                sum += i;
            }
        }

        chain_len[i] = chain_len[ totients[i] ] + 1;
    }
    println!("{}", sum);
}
