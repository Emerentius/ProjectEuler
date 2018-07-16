extern crate num; // 0.2.0
use num::rational::Ratio;
use std::collections::{HashMap, HashSet};

// y = mx + b
enum Line {
    Vertical{ x: i64 },
    NonVertical {
        m: Ratio<i64>,
        b: Ratio<i64>
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
enum LineKey {
    Vertical,
    NonVertical { m: Ratio<i64> }
}

fn points_seq() -> impl Iterator<Item=(i64, i64)> {
    let s0 = 290797i64;
    let next_s = |n| n*n % 50515093;
    let t = |s| (s % 2000) - 1000;
    std::iter::repeat(()).scan(s0, move |s_prev, ()| {
        let s1 = next_s(*s_prev);
        let s2 = next_s(s1);
        *s_prev = s2;
        Some((t(s1), t(s2)))
    })
}

fn line(p1: (i64, i64), p2: (i64, i64)) -> Line {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    if dx == 0 {
        Line::Vertical{ x: p1.0 }
    } else {
        let m = Ratio::new(dy, dx);
        let b = Ratio::new(p1.1, 1) - m*p1.0;
        Line::NonVertical{ m, b }
    }
}

fn main() {
    let mut points = HashSet::new();
    let mut lines = HashMap::new();

    let mut n_lines = 0;     // M(L)
    let mut n_crossings = 0; // S(L)

    for point in points_seq().take(2500) {
        if points.contains(&point) { continue }
        for &prev_point in points.iter() {
            let (entry, id) = match line(point, prev_point) {
                Line::NonVertical { m, b } => {
                    (lines.entry(LineKey::NonVertical { m }), b)
                }
                Line::Vertical { x } => {
                    (lines.entry(LineKey::Vertical), Ratio::new(x, 1))
                }
            };
            let parallel_lines = entry.or_insert_with(Vec::new);
            if !parallel_lines.contains(&id) {
                parallel_lines.push(id);
                n_lines += 1;
                let n_nonparallel_lines = n_lines - parallel_lines.len();
                n_crossings += 2*n_nonparallel_lines;
            }
        }

        points.insert(point);
    }
    println!("m: {}, s: {}", n_lines, n_crossings);
}
