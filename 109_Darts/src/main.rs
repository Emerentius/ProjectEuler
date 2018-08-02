#![feature(test)]
extern crate test;

fn is_final_double(n: i32) -> bool {
    let (half, rest) = (n / 2, n % 2);
    rest == 0 && ((half >= 1 && half <= 20) || half == 25)
}

fn count_possibilities(n_throws: u8, remaining: i32, scores: &[i32]) -> usize {
    let mut sum = if is_final_double(remaining) { 1 } else { 0 };
    if n_throws < 2 {
        sum += scores.iter()
            .take_while(|&&score| remaining - score >= 0) // purely for optimization
            .enumerate()
            .map(|(i, &score)| count_possibilities(n_throws + 1, remaining-score, &scores[i..]) )
            .sum::<usize>();
    }
    sum
}

fn main() {
    let mut possible_scores: Vec<_> = (1..=20)
        .chain( (1..=20).map(|n| n*2))
        .chain( (1..=20).map(|n| n*3))
        .chain([25, 50].iter().cloned())
        .collect();
    possible_scores.sort(); // purely for optimization
    println!("{}", (1..100).map(|total| count_possibilities(0, total, &possible_scores)).sum::<usize>());
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
