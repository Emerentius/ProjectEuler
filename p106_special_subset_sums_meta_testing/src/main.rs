#![feature(test)]
extern crate test;

const MAX_SIZE: usize = 12;

fn main() {
    // List of the numbers from smallest to highest [a, b, c, d, ..], represented
    // by numbers.
    // The value in the vec is their place in the order, not the actual value.
    //
    // If you pick subsets like [a, b] and another [c, d],
    // then obviously [c,d] is larger, because you can pair off the numbers
    // a < c
    // b < d
    // => a + b < c + d
    let orderings = (1..=MAX_SIZE as u8).collect::<Vec<_>>();

    let mut n_total = 0;
    let mut num_taken = [false; MAX_SIZE];
    let mut num1 = vec![];
    let mut num2 = vec![];
    for size in 1..=MAX_SIZE / 2 {
        n_total += walk_subsets(
            size,
            0,
            &orderings,
            &mut num_taken,
            &mut num1,
            &mut num2,
            true,
        );
    }

    println!("{}", n_total / 2);
}

// visit all pairs of subsets in a depth-first search.
// First, generate one subset, then the other and then check
// if one is trivially either larger or smaller than the other based on
// the position of the numbers in the size order.
// Sum all situations that aren't obvious. This overcounts by a factor of 2
// because for each pair the inverse pair is also visited.
//
// pretty ugly.
fn walk_subsets(
    target_size: usize,
    i_start: usize,
    orderings: &[u8],
    num_taken: &mut [bool],
    num1: &mut Vec<u8>,
    num2: &mut Vec<u8>,
    first: bool,
) -> usize {
    let target_num = if first { &mut *num1 } else { &mut *num2 };
    if target_num.len() == target_size {
        if first {
            return walk_subsets(target_size, 0, orderings, num_taken, num1, num2, false);
        } else if num1.iter().zip(num2.iter()).all(|(n1, n2)| n1 < n2)
            || num1.iter().zip(num2.iter()).all(|(n1, n2)| n1 > n2)
        {
            return 0;
        } else {
            return 1;
        }
    }

    let mut sum = 0;
    for i in i_start..MAX_SIZE {
        if num_taken[i] {
            continue;
        }
        let chosen_num = orderings[i];

        let target_num = if first { &mut *num1 } else { &mut *num2 };
        target_num.push(chosen_num);
        num_taken[i] = true;

        sum += walk_subsets(target_size, i + 1, orderings, num_taken, num1, num2, first);

        num_taken[i] = false;
        let target_num = if first { &mut *num1 } else { &mut *num2 };
        target_num.pop();
    }
    sum
}

#[cfg(test)]
#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(main)
}
