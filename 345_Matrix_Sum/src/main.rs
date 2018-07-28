#![feature(test)]
#![feature(collections)]
extern crate test;

fn find_max_sum (slice: &[Vec<(u32, u32)>], cols_seen: [bool;15]) -> Option<(u32, [bool;15])> {
    //print!("{}:", 16-slice.len());
    match slice.first() {
        None => return Some((0, cols_seen)), // recursion ends
        Some(row) => {
            let tail = slice.tail();
            let mut results = vec![];
            for &(num, col) in row.iter() {
                //println!("\t({},{})", num, col);
                if cols_seen[col as usize] { continue }
                let mut loc_cols_seen = cols_seen;
                loc_cols_seen[col as usize] = true;
                if let Some((num2, new_cols_seen)) = find_max_sum(tail, loc_cols_seen) {
                    results.push( (num+num2, new_cols_seen) );
                }
            }
            results.sort();
            if let Some(&tuple) = results.last() {
                return Some(tuple)
            }
        },
    }
    None
}

fn main() {
    let matrix = vec![
    7,53,183,439,863,497,383,563,79,973,287,63,343,169,583,
    627,343,773,959,943,767,473,103,699,303,957,703,583,639,913,
    447,283,463,29,23,487,463,993,119,883,327,493,423,159,743,
    217,623,3,399,853,407,103,983,89,463,290,516,212,462,350,
    960,376,682,962,300,780,486,502,912,800,250,346,172,812,350,
    870,456,192,162,593,473,915,45,989,873,823,965,425,329,803,
    973,965,905,919,133,673,665,235,509,613,673,815,165,992,326,
    322,148,972,962,286,255,941,541,265,323,925,281,601,95,973,
    445,721,11,525,473,65,511,164,138,672,18,428,154,448,848,
    414,456,310,312,798,104,566,520,302,248,694,976,430,392,198,
    184,829,373,181,631,101,969,613,840,740,778,458,284,760,390,
    821,461,843,513,17,901,711,993,293,157,274,94,192,156,574,
    34,124,4,878,450,476,712,914,838,669,875,299,823,329,699,
    815,559,813,459,522,788,168,586,966,232,308,833,251,631,107,
    813,883,451,509,615,77,281,613,459,205,380,274,302,35,805
    ];
    let mut rows = vec![];
    for chunk in matrix.chunks(15) {
        let mut next_row = vec![];
        next_row.push_all(chunk);

        let mut sorted_row : Vec<_> = next_row.into_iter().zip(0..15).collect();
        sorted_row.sort_by(|a,b| a.cmp(b).reverse());
        rows.push(sorted_row);
    }

    let mut removing = true;
    while removing {
        removing = false;
        for n_row in 0..rows.len() {
            for n_col in 0..(rows[n_row]).len() {
                let (num, real_col) = rows[n_row][n_col];
                for n_row2 in (0..rows.len()).filter(|&n| n != n_row) {
                    // the 3 is a guess, the algorithm is faulty
                    for n_col2 in (n_col+3..(rows[n_row2]).len()).rev() {
                        let (num2, real_col2) = rows[n_row2][n_col2];
                        if real_col == real_col2 && num > num2 {
                            (rows[n_row2]).remove(n_col2);
                            removing = true;
                        }
                    }
                }
            }
        }
    }

    /*for row in rows {
        println!("{:?}", row);
    }*/

    println!("{:?}", find_max_sum(&rows, [false;15]).unwrap().0);
}

#[bench]
fn bench (b: &mut test::Bencher) {
    b.iter(|| main())
}
