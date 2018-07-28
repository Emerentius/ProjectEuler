use std::cmp;

fn recurse_left(mut sum: i64, triangle: &Vec<Vec<i64>>) -> i64 {
    let mut min = recurse_right(0, sum, triangle);
    for l in 0..N {
        for line in &triangle[l..N] {
            sum -= line[l];
        }
        min = cmp::min(min, sum);
        min = cmp::min(min, recurse_right(l+1, sum, triangle));
    }
    min
}

fn recurse_right(l: usize, mut sum: i64, triangle: &Vec<Vec<i64>>) -> i64 {
    let mut min = recurse_bottom((l, 0), sum, triangle);
    for r in 0..N-l {
        for line in &triangle[l+r..N] {
            sum -= line[line.len()-1-r];
        }
        min = cmp::min(min, sum);
        min = cmp::min(min, recurse_bottom((l, r+1), sum, triangle));
    }
    min
}

fn recurse_bottom((l, r): (usize, usize), mut sum: i64, triangle: &Vec<Vec<i64>>) -> i64 {
    let mut min = sum;
    for b in 0..N-l-r {
        let line = &triangle[N-1-b];
        sum -= (&line[l..line.len()-r]).iter().sum::<i64>();
        min = cmp::min(min, sum);
    }
    min
}

const N: usize = 1000;

fn main() {
    let mod_ = 2i64.pow(20);
    let diff = 2i64.pow(19);

    let mut triangle = vec![];
    let mut t = 0;
    let mut total_triangle_sum = 0;
    for line_length in 1..N+1 {
        let mut line = vec![];
        for _ in 0..line_length {
            t = (615949*t + 797807) % mod_;
            line.push(t - diff);
            total_triangle_sum += t - diff;
        }
        triangle.push(line);
    }

    let min = recurse_left(total_triangle_sum, &triangle);
    println!("{}", min);
}
