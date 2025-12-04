use std::collections::HashSet;
use crate::util;

/// Print the solutions to day 4 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let part1;
    // Load the grid as a HashMap
    let (grid, width, height) = util::read_grid_map(lines, '.').unwrap();
    let mut removed_rolls:HashSet<util::Vec2> = HashSet::new();
    // Iterate each non-default position, counting neighbors of each roll and 
    // marking them as removed if removable.
    for pos in grid.keys() {
        let mut neighbor_count = 0;
        for dir in util::adjacent8() {
            let neighbor = *pos + dir;
            if neighbor.in_bounds(width, height) && *grid.get(&neighbor) == '@' {
                neighbor_count += 1;
            }
        }
        if neighbor_count < 4 {
            removed_rolls.insert(pos.clone());
        }
    }
    part1 = removed_rolls.len();
    // Continue to iterate over rolls, marking newly removable ones as 
    // removable. Stop when no more new removable rolls are found.
    loop {
        let mut removed = 0;
        for pos in grid.keys() {
            if removed_rolls.contains(pos) {
                continue;
            }
            let mut neighbor_count = 0;
            for dir in util::adjacent8() {
                let neighbor = *pos + dir;
                if neighbor.in_bounds(width, height) 
                    && grid.contains_key(&neighbor) 
                    && *grid.get(&neighbor) == '@' 
                    && !removed_rolls.contains(&neighbor) {
                    neighbor_count += 1;
                }
            }
            if neighbor_count < 4 {
                removed += 1;
                removed_rolls.insert(pos.clone());
            }
        }
        if removed == 0 { break; }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", removed_rolls.len());
}
