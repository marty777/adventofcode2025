use crate::util;

/// Print the solutions to day 5 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let mut part2 = 0;
    let sections = util::sections(lines);
    let mut fresh_ids = Vec::new();
    for i in 0..sections[0].len() {
        let parts = sections[0][i].split("-").collect::<Vec<&str>>();
        let start = parts[0].parse::<usize>().unwrap();
        let stop = parts[1].parse::<usize>().unwrap();
        fresh_ids.push((start, stop));
    }
    // Part 1 - Count fresh ingredients
    for i in 0..sections[1].len() {
        let ingredient = sections[1][i].parse::<usize>().unwrap();
        for j in 0..fresh_ids.len() {
            if ingredient >= fresh_ids[j].0 && ingredient <= fresh_ids[j].1 {
                part1 += 1;
                break;
            }
        }
    }
    // Part 2 - Merge fresh id ranges until no more merges are possible, then 
    // sum the range widths
    loop {
        let mut next_fresh_ids:Vec<(usize, usize)> = Vec::new();
        let mut merges = 0;
        for i in 0..fresh_ids.len() {
            let mut merged = false;
            for j in 0..next_fresh_ids.len() {
                // If the fresh id range contains a previously added range, 
                // expand the range.
                if fresh_ids[i].0 <= next_fresh_ids[j].0 && fresh_ids[i].1 >= next_fresh_ids[j].1 {
                    next_fresh_ids[j].0 = fresh_ids[i].0;
                    next_fresh_ids[j].1 = fresh_ids[i].1;
                    merged = true;
                    break;
                } 
                // If the fresh id range is contained by a previously added 
                // range, do not expand the range.
                if fresh_ids[i].0 >= next_fresh_ids[j].0 && fresh_ids[i].1 <= next_fresh_ids[j].1 {
                    merged = true;
                    break;
                } 
                // If the fresh id range overlaps a previously added range 
                // on the left, expand the range to the left.
                if fresh_ids[i].0 <= next_fresh_ids[j].0 && fresh_ids[i].1 >= next_fresh_ids[j].0 && fresh_ids[i].1 <= next_fresh_ids[j].1 {
                    next_fresh_ids[j].0 = fresh_ids[i].0;
                    merged = true;
                    break;
                } 
                // If the fresh id range overlaps a previously added range 
                // on the right, expand the range to the right.
                if fresh_ids[i].0 >= next_fresh_ids[j].0 && fresh_ids[i].0 <= next_fresh_ids[j].1 && fresh_ids[i].1 >= next_fresh_ids[j].1 {
                    next_fresh_ids[j].1 = fresh_ids[i].1;
                    merged = true;
                    break;
                } 
            }
            if merged {
                merges += 1;
            }
            else {
                next_fresh_ids.push(fresh_ids[i]);
            }
        }
        fresh_ids = next_fresh_ids;
        if merges == 0 {
            break;
        }
    }
    for i in 0..fresh_ids.len() {
        part2 +=  fresh_ids[i].1 -fresh_ids[i].0 + 1;
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
