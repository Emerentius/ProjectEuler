fn factorial(arg:u64) -> u64 {
    [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880][arg as usize]
}

fn digit_factorial_sum(mut arg:u64) -> u64 {
    let mut sum = 0;
    while arg != 0 {
        sum += factorial(arg % 10);
        arg /= 10;
    }
    sum
}

fn main() {
    let mut looping = vec![169, 363601, 1454, 871, 45361, 872, 45362];

    let mut n_60_long_chains = 0;
    for mut i in 1..1_000_000u64 {
        let mut chain_length = 1;
        while !looping.contains(&i)  {
            let dfs = digit_factorial_sum(i);
            if i == dfs { looping.push(i) }
            i = dfs;
            chain_length += 1;
        }
        match i {
            169 | 363601 | 1454 => chain_length += 2,
            871 | 45361 | 872 | 45362 => chain_length += 1,
            _ => (),
        }

        if chain_length == 60 { n_60_long_chains += 1 }
    }
    println!("{}", n_60_long_chains);
}
