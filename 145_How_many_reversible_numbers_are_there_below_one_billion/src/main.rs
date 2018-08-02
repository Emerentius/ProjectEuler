fn odd_digits_only (mut num : u64) -> bool {
    while num != 0 {
        if num % 2 == 0 { return false }
        num /= 10;
    }
    true
}

fn reverse (mut num: u64) -> u64 {
    let mut new_num = 0;
    while num != 0 {
        new_num = new_num*10 + num % 10;
        num /= 10;
    }
    new_num
}

fn main() {
    let count = (1..1_000_000_000)
        .filter(|&n| n%10 != 0 && odd_digits_only(n+reverse(n)))
        .count();
    println!("{}", count);
}
