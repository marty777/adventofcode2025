use crate::util;

/// Validate the given `id` against the part 1 or part 2 rules
fn valid(id:usize, part1:bool) -> bool {
    let digits = util::base_n_digits(id as i128, 10, None);
    if part1 && digits.len() % 2 != 0 {
        return true;
    }
    for window in (1..=digits.len() / 2).rev() {
        if digits.len() % window == 0 {
            let repeats = digits.len() / window;
            let mut has_repeat = true;
            for i in 0..window {
                for j in 1..repeats {
                    if digits[i] != digits[i + j * window] {
                        has_repeat = false;
                        break;
                    }
                }
            }
            if has_repeat { return false; }
        }
        if part1 { break; }
    }
    return true;
}
/// Print the solutions to day 2 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let mut part2 = 0;
    let ranges:Vec<&str> = lines[0].split(",").collect();
    for i in 0..ranges.len() {
        let parts:Vec<&str> = ranges[i].split("-").collect();
        let start = parts[0].parse::<usize>().unwrap();
        let stop = parts[1].parse::<usize>().unwrap();
        for i in start..=stop {
            if !valid(i, true) {
                part1 += i;
            }
            if !valid(i, false) {
                part2 += i;
            }
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
