extern crate euler_tools;
use euler_tools::PermutationsStreamIter; // Iterator over all permutations

fn next_carriage(carr: char) -> char {
    (carr as u8 + 1) as char
}

fn move_carriage_to_end(carriages: &mut [char], carriage: char) {
    if carriage == *(carriages.first().unwrap()) {
        carriages.reverse();
    } else {
        move_carriage_to_end(&mut carriages[1..], carriage)
    }
}

// return amount of switches needed
fn solve_inefficienctly(carriages: &mut [char], carriage: char) -> u32 {
    let mut move_count = 0;
    match carriages.first().cloned() {
        None => 0,
        Some(first) => {
            if first != carriage { // move it to first
                if carriages.last().cloned().unwrap() != carriage {
                    move_carriage_to_end(&mut carriages[1..], carriage);
                    move_count += 1;
                }
                carriages.reverse();
                move_count += 1;
            }
            move_count + solve_inefficienctly(&mut carriages[1..], next_carriage(carriage))
        }
    }
}

const N_CARRIAGES: u8 = 11;
const NTH_MAXIMIX: usize = 2011;

fn main() {
    let chars = (b'A'..b'A'+N_CARRIAGES)
        .map(|n| n as char)
        .collect::<Vec<_>>();

    let mut permutations = PermutationsStreamIter::new(chars);
    let mut n_switches_max = 0;
    let mut maximix_arrangements = vec![];

    while let Some(arrangement) = permutations.streaming_next() {
        let n_switches = solve_inefficienctly(&mut arrangement.to_owned(), 'A');
        if n_switches > n_switches_max {
            n_switches_max = n_switches;
            maximix_arrangements.clear();
        }
        if n_switches == n_switches_max {
            maximix_arrangements.push(arrangement.to_owned());
        }
    }

    // print answer
    for el in &maximix_arrangements[NTH_MAXIMIX - 1] {
        print!("{}", el);
    }
}
