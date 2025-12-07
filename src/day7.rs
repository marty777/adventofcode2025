use crate::util;

/// Print the solutions to day 7 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    // Read the grid
    let grid = util::read_grid(lines);
    let width = grid[0].len();
    let height = grid.len();
    // Get the S position
    let s_x = lines[0].find("S").unwrap();
    // Keep a sum of total beams that reach each x position
    let mut beams:Vec<usize> = vec![0;width];
    beams[s_x] = 1;
    for y in 1..height {
        // Update the beams at each x position at this depth of the grid.
        let mut next_beams = vec![0;width];
        for x in 0..width {
            if grid[y][x] == '^' {
                // If a beam reaches this splitter, increment the part 1 count.
                if beams[x] > 0 {
                    part1 += 1;
                }
                if x > 0 {
                    next_beams[x-1] += beams[x];
                }
                if x < width - 1 {
                    next_beams[x+1] += beams[x];
                }
            }
            else {
                next_beams[x] += beams[x];
            }
        }
        beams = next_beams;
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", beams.iter().sum::<usize>());
}
