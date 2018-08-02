#![feature(test)]
extern crate test;

const N: u8 = 5;
const N_POW: u8 = 32; // 2^N

fn find_solutions(n_subseq: u8, prev_seq: &mut [bool; 32], stack: &mut Vec<u8>, sols: &mut Vec<u32>) {
    if n_subseq == N_POW - 1 { // -1, because 0 is known before hand
        sols.push(
            stack.iter()
                .fold(0, |num, &subseq| (num << 1) | (subseq >> (N-1)) as u32)
        )
    }
    let prev: u8 = stack.last().unwrap().clone();

    for next in [0,1].iter().map(|&bit| ((prev << 1) % N_POW) | bit) {
        if !prev_seq[next as usize] {
            prev_seq[next as usize] = true;
            stack.push(next);
            find_solutions(n_subseq+1, prev_seq, stack, sols);
            stack.pop();
            prev_seq[next as usize] = false;
        }
    }
}

fn main() {
    let mut subsequence_stack = vec![0u8];
    let mut solutions: Vec<u32> = vec![];
    let mut prev_seq = [false; 32];
    prev_seq[0] = true;
    find_solutions(0, &mut prev_seq, &mut subsequence_stack, &mut solutions);
    println!("{}",
        solutions.into_iter().fold(0, |sum, el| sum + el as u64)
    );
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
