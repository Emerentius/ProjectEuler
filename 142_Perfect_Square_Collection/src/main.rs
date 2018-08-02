use std::collections::BTreeSet;

fn main() {
    let squares_set : BTreeSet<i32> = (1..1000+1).map(|n| n*n).collect();
    let is_square = |num| squares_set.contains(&num);
    let squares : Vec<i32> = (1..1000+1).map(|n| n*n).collect();

    'x: for x in 1..1_000_000 {
        for y in squares.iter().map(|sq| x-sq)
            .take_while(|&y| y >= 0)
            .filter(|&y| is_square(x+y)) {
            for z in squares.iter().map(|sq| y-sq).take_while(|&z| z >= 0) {
                if is_square(x+z) && is_square(x-z)
                && is_square(y+z) {
                    println!("{}+{}+{} = {}", x,y,z, x+y+z);
                    break 'x
                }
            }
        }
    }
}
