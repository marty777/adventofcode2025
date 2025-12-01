
use crate::util;

/// Print the solutions to day 1 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut pos = 50;
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..lines.len() {
        let (prefix, num_str) = lines[i].split_at(1);
        let number = num_str.parse::<i128>().unwrap();
        // For the positive direction, the number of times the zero is crossed 
        // is simple division to complement the modular remainder for the 
        // dial position.
        if prefix == "R" {
            part2 += (pos + number) / 100;
            pos = (pos + number) % 100;
        }
        // The negative direction is more complicated, but treating the dial as
        // if it were inverted to count the zero crossings in the positive 
        // direction simplifies things
        else {
            part2 += (((100 - pos) % 100) + number) / 100;
            pos = util::abs_mod(pos - number, 100);
        }
        if pos == 0 {
            part1 += 1;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
