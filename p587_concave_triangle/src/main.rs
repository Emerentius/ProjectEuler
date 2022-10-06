#![feature(test)]
extern crate test;

fn x_intersection(n: u64) -> f64 {
    let n = n as f64;
    let a = 1.0 + 1.0 / n;
    let b = 1.0 / (1.0 + 1.0 / (n*n) );

    let p_half = -a * b;
    let q = b;
    -p_half - f64::sqrt(p_half*p_half - q)
}

fn integral(x_int: f64, n: u64) -> f64 {
    let slope = 1.0 / n as f64;
    // triangle part
    let y_int = slope * x_int;
    let int_left = 0.5 * y_int * x_int;
    // circle part
    // analytic formula derived by hand
    let int_right = 1.0 - x_int + 0.5 * (f64::sqrt(2.0*x_int - x_int*x_int)*(x_int - 1.0) + f64::asin(x_int - 1.0));

    int_left + int_right
}

fn main() {
    let area_l_section = (4.0 - std::f64::consts::PI) / 4.0;
    let area_percentage = |n| {
        let x_intersection = x_intersection(n);
        integral(x_intersection, n) / area_l_section
    };

    let solution = (2..)
        .find(|&n| area_percentage(n) < 0.001)
        .unwrap();

    print!("{}", solution);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
