#![feature(test)]
extern crate test;

fn is_palindrome(num: u64) -> bool {
    let mut num_tmp = num;
    let mut reversed = 0;
    while num_tmp != 0 {
        reversed = reversed*10 + num_tmp % 10;
        num_tmp /= 10;
    }
    reversed == num
}

fn main() {
    let squares : Vec<_> = (1..10000).map(|n| n*n).collect();
    let mut palindromes = vec![];

    for start in 0..squares.len() - 1 {
        let mut square_sum = squares[start];
        for &square in &squares[start+1..] {
            square_sum += square;
            if square_sum > 100_000_000 { break }
            if is_palindrome(square_sum) {
                palindromes.push(square_sum);
            }
        }
    }
    palindromes.sort();
    palindromes.dedup();
    let sum = palindromes.into_iter().fold(0, |sum, p| sum + p);
    println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
