fn f(x: f64) -> f64 {
    (2.0f64).powf(30.403243784-x*x).floor() * 1e-9
}

fn main() {
    let mut u = -1.0;
    let mut last_u = 0.0;
    for i in 0..1_000+1 {
        last_u = u;
        u = f(u);
        if i % 100 == 0 { println!("{}", u+last_u); }
    }
    println!("{}", u+last_u);
}
