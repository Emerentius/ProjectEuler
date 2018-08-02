fn digits(mut num: u32) -> Vec<u32> {
    let mut digits = vec![];
    while num != 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

fn main() {
    let keylog = include_str!("p079_keylog.txt");
    let key_parts = keylog.split_whitespace()
        .map(|num| {
            num.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    'key: for key in 100.. {
        let key_digs = digits(key);
        for key_subset in &key_parts {
            let mut key_iter = key_digs.iter().rev();
            for &dig in key_subset {
                if key_iter.find(|&&e| dig == e).is_none() { continue 'key }
            }
        }
        println!("{}", key);
        break
    }
}
