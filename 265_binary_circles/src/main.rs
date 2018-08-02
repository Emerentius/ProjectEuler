#![feature(test)]
extern crate test;

const N: u8 = 5;
const N_POW: u8 = 32; // 2^N

fn find_solutions(n_subseq: u8, stack: &mut Vec<u8>, sols: &mut Vec<u32>) {
    if n_subseq == N_POW - 1 { // -1, because 0 is known before hand
        let sol = stack.iter()
            .fold(0, |num, &seq| (num << 1) | (seq >> (N-1)) as u32);
        sols.push(sol);
    }
    let prev: u8 = stack.last().unwrap().clone();

    for next in [0,1].iter().map(|&bit| ((prev << 1) % N_POW) | bit) {
        if !stack.contains(&next) {
            stack.push(next);
            find_solutions(n_subseq+1, stack, sols);
            stack.pop();
        }
    }
}

fn main() {
    let mut subsequence_stack = vec![0u8];
    let mut solutions: Vec<u32> = vec![];
    find_solutions(0, &mut subsequence_stack, &mut solutions);
    println!("{}",
        solutions.into_iter().fold(0, |sum, el| sum + el as u64)
    );
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
