// (b/2)^2 + h^2 = L^2
// h = b ± 1
// => b, h
// => b,h, L coprime
// b even
// h odd
//
// Pythagorean triplets can be used to find candidates
// b/2 = 2*m*n <=> b = 4*m*n
// h = m^2 - n^2
// L = m^2 + n^2
// h - b = m^2 - n^2 - 4 * m *n

fn main() {
    let mut m: i128 = 2;
    let mut n: i128 = 1;
    let mut count = 0;
    let mut sum = 0;
    loop {
        // b/2 < h
        let b = 4 * m * n;
        let h = m * m - n * n;
        let diff = h - b;

        if diff.abs() == 1 {
            let l = m * m + n * n;
            println!("{count}: {b}, {h}, {l}");
            count += 1;
            m *= 4;
            n *= 4;
            sum += l;
        } else if diff < 0 {
            m += 1;
        } else {
            n += 1;
        }
        if count == 12 {
            break;
        }
    }
    println!("{sum}");
}
