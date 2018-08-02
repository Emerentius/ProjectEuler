extern crate num;
use num::Rational;
use std::collections::BTreeSet;

fn main() {
    // circuits[n_capacitors-1] = collection of capacitances for n capacitors
    let mut circuits = vec![vec![Rational::new(1,1)]];
    // set of all capacitances possible
    let mut all_capacitances = BTreeSet::new();
    all_capacitances.insert(Rational::new(1,1));

    while circuits.len() < 18 {
        let mut capacitances = vec![];

        let n = circuits.len();

        // iterate through all capacitances for 1+n, 2+(n-1), 3+(n-2).. capacitors
        // and combine them
        for (caps1, caps2) in Iterator::zip(circuits.iter(), circuits.iter().rev() )
            .take( (n+1)/2) // no duplicate calculations
        {
            for &cap1 in caps1 {
                for &cap2 in caps2 {
                    capacitances.push(cap1+cap2);
                    capacitances.push(cap1*cap2/(cap1+cap2));
                }
            }
        }

        // delete duplicates and suboptimal networks
        capacitances.sort();
        capacitances.dedup();
        capacitances.retain(|&cap| !all_capacitances.contains(&cap));

        all_capacitances.extend(capacitances.iter().cloned() );

        circuits.push(capacitances);
    }

    println!("{:?}", all_capacitances.len());
}
