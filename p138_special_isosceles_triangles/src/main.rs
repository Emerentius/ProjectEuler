use euler_utils::num::IntSqrt;

fn main() {
    let mut solutions = vec![];
    for b in (2u64..4_000_000_000).step_by(2) {
        let x = b * b / 4 * 5 + 1;
        let l_squared_candidates = [x + 2 * b, x - 2 * b];
        for l in l_squared_candidates {
            if let Some(l) = l.sqrt() {
                println!("{b}, {l}",);
                solutions.push((b, l));
            }
        }
    }

    let factors = solutions[1..]
        .iter()
        .zip(&solutions)
        .map(|(&(next_b, next_l), &(prev_b, prev_l))| {
            (next_b as f64 / prev_b as f64, next_l as f64 / prev_l as f64)
        })
        .collect::<Vec<_>>();
    println!("{:?}", factors);
}
