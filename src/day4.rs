use std::collections::HashSet;
use crate::util;

/// Print the solutions to day 4 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let mut part2 = 0;
    // Load the grid as a DefaultHashMap
    let (grid, width, height) = util::read_grid_map(lines, '.').unwrap();
    // Get all initial roll positions in the grid
    let mut remaining_rolls:HashSet<&util::Vec2> = HashSet::from_iter(grid.keys().into_iter());
    // In a loop, iterate over rolls to find all rolls that can currently be 
    // removed. If no rolls are marked as removable, exit the loop. Otherwise,
    // remove the marked rolls and continue with the next pass, recording the 
    // number of removed items for the part 1 and part 2 answers.
    loop {
        let mut removed_in_loop = HashSet::new();
        for pos in remaining_rolls.iter() {
            let mut neighbor_count = 0;
            for dir in util::adjacent8() {
                let neighbor = **pos + dir;
                // Count the neighbor if it's a roll and it hasn't been 
                // previously removed
                if neighbor.in_bounds(width, height) 
                    && remaining_rolls.contains(&neighbor) {
                    neighbor_count += 1;
                }
            }
            if neighbor_count < 4 {
                removed_in_loop.insert((*pos).clone());
            }
        }
        if part1 == 0 { part1 = removed_in_loop.len(); }
        part2 += removed_in_loop.len();
        // remove marked rolls from remaining rolls
        remaining_rolls.retain(|&p| !removed_in_loop.contains(p));
        if removed_in_loop.len() == 0 { break; }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
