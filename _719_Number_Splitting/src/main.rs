fn is_s_number(n: u64, n_square: u64) -> bool {
    _is_s_number(n, n_square, 0, 1, 0)
}

// recurse through all digits in n_square from least to most significant
// and decide after each digit whether to continue grouping digits into a larger number
// or to split (depth-first).
fn _is_s_number(n: u64, square_rest: u64, mut current_num: u64, multiplier: u64, sum: u64) -> bool {
    if square_rest == 0 {
        return n == sum + current_num;
    }

    let last_digit = square_rest % 10;
    current_num = current_num + last_digit * multiplier;

    let rest = square_rest / 10;

    _is_s_number(n, rest, current_num, multiplier * 10, sum)
        || _is_s_number(n, rest, 0, 1, sum + current_num)
}

fn main() {
    let limit = 10u64.pow(12);
    // skipping 1 as it's the only number that would fit the check
    // but it can't be split into 2 numbers for this so it doesn't count
    let perfect_squares = (2..)
        .map(|n| (n, n * n))
        .take_while(|&(_, square)| square <= limit);
    let total_sum = perfect_squares
        .filter(|&(n, n_square)| is_s_number(n, n_square))
        .map(|(_, n_square)| n_square)
        .sum::<u64>();

    println!("{}", total_sum);
}
