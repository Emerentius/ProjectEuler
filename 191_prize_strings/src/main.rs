fn number_of_prizes(day: u8, was_absent: (bool, bool), was_late: bool) -> u32
{
    // base case
    // 1...30 period over
    if day == 31 {
        return 1
    }

    let mut sum = 0;
    // Absent today
    if was_absent != (true, true) {
        sum += number_of_prizes(day+1, (was_absent.1, true), was_late);
    }

    // is late today
    if !was_late {
        sum += number_of_prizes(day+1, (was_absent.1, false), true);
    }

    // On time
    sum += number_of_prizes(day+1, (was_absent.1, false), was_late);
    sum
}

fn main() {
    println!("{}", number_of_prizes(1, (false, false), false));
}
