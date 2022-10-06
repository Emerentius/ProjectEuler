fn digits (mut num: u64) -> [u64; 10] {
    let mut digits = [0;10];
    while num != 0 {
        digits[(num%10) as usize] += 1;
        num /= 10;
    }
    digits
}

// this is pure brute force
// takes a few hours
fn main() {
    let mut fs_n = [0u64;10];

    let mut sum = 0;
    for n in 1.. {
        if n % 1_000_000_000 == 0 {println!("{}:\t{}", n, sum) }
        let digits = digits(n);
        for (i,&digit) in digits.iter().enumerate().skip(1) {
            fs_n[i] += digit;
        }
        for &f in fs_n.iter().skip(1) {
            if f == n { sum += n }
        }
    }
    println!("{}", sum);
}
