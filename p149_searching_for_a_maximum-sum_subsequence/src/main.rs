use std::collections::VecDeque;

fn lagged_fibonacci() -> impl Iterator<Item = i64> {
    const M: i64 = 1000000;
    const OFF: i64 = 500000;
    let mut prev_nums = (1..56i64)
        .map(|k| (100003 - 200003 * k + 300007 * k.pow(3)) % M - OFF)
        .collect::<VecDeque<_>>();

    std::iter::from_fn(move || {
        let next_random_number =
            (prev_nums[prev_nums.len() - 24] + prev_nums[prev_nums.len() - 55] + M) % M - OFF;
        prev_nums.push_back(next_random_number);
        prev_nums.pop_front()
    })
}

// Kadane's algorithm
fn maximum_subsum(numbers: impl IntoIterator<Item = i64>) -> i64 {
    let mut current_sum = 0;
    numbers
        .into_iter()
        .map(|number| {
            current_sum = std::cmp::max(0, current_sum + number);
            current_sum
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    // row, column indexing by hand
    let table = lagged_fibonacci().take(4_000_000).collect::<Vec<_>>();

    let idx_table = |row, col| table[(row * 2000 + col) as usize];

    // create index iterators for all possible directions and compute the maximum subsum
    // for each one.
    // `maximum_subsum` is O(n) and there's `O(n)` subsequences of length `O(n)`
    // => total complexity O(n²)
    let horizontal_max_subsum = (0..2000)
        .map(|row| maximum_subsum((0..2000).map(|col| idx_table(row, col))))
        .max()
        .unwrap();
    let vertical_max_subsum = (0..2000)
        .map(|col| maximum_subsum((0..2000).map(|row| idx_table(row, col))))
        .max()
        .unwrap();

    let down_diagonal_max = (0..2000)
        .map(|start_row| (start_row, 0))
        .chain((0..2000).map(|start_col| (0, start_col)))
        .map(|(start_row, start_col)| {
            let iter = (start_row..2000)
                .zip(start_col..2000)
                .map(|(row, col)| idx_table(row, col));
            maximum_subsum(iter)
        })
        .max()
        .unwrap();

    let up_diagonal_max = (0..2000)
        .map(|start_row| (start_row, 0))
        .chain((0..2000).map(|start_col| (2000 - 1, start_col)))
        .map(|(start_row, start_col)| {
            let iter = (0..start_row + 1)
                .rev()
                .zip(start_col..2000)
                .map(|(row, col)| idx_table(row, col));
            maximum_subsum(iter)
        })
        .max()
        .unwrap();

    let maxima = [
        horizontal_max_subsum,
        vertical_max_subsum,
        up_diagonal_max,
        down_diagonal_max,
    ];
    let total_max = maxima.iter().cloned().max().unwrap();
    println!("{}", total_max);
}

#[test]
fn test_lagged_fibonacci() {
    assert_eq!(lagged_fibonacci().nth(10 - 1), Some(-393027));
    assert_eq!(lagged_fibonacci().nth(100 - 1), Some(86613));
}
