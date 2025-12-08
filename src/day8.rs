use crate::util;
use crate::util::Vec3;
use std::collections::HashSet;

pub fn merge(groups:&mut Vec<HashSet<usize>>, a:usize, b:usize) {
    let mut a_group_index:Option<usize> = None;
    let mut b_group_index:Option<usize> = None;
    for i in 0..groups.len() {
        if groups[i].contains(&a) {
            a_group_index = Some(i);
        }
        if groups[i].contains(&b) {
            b_group_index = Some(i);
        }
    }
    assert!(!a_group_index.is_none(), "Merge of indexes {}, {} failed. {} not found in groups", a, b, a);
    assert!(!b_group_index.is_none(), "Merge of indexes {}, {} failed. {} not found in groups", a, b, b);
    // If a,b are already in the same group, done
    if a_group_index == b_group_index {
        return;
    }
    // If a,b are in separate groups, remove one group and add it to the other
    else {
        let b_group = groups.remove(b_group_index.unwrap());
        if a_group_index.unwrap() > b_group_index.unwrap() {
            groups[a_group_index.unwrap() - 1].extend(b_group);
        }
        else {
            groups[a_group_index.unwrap()].extend(b_group);    
        }
    }
}

/// Print the solutions to day 8 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 1;
    let mut part2 = 0;
    let numbers = util::ints_in_strings(lines);
    let mut positions = Vec::new();
    for i in 0..numbers.len() {
        positions.push(Vec3::new(numbers[i][0],numbers[i][1],numbers[i][2]));
    }
    // Construct a list distances between all positions then sort by ascending distance
    let mut dists:Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            dists.push(((positions[i] - positions[j]).len_squared() as usize, i, j));
        }
    }
    dists.sort_by(|x, y| (x.0).cmp(&y.0));
    // Set limit of connections to add for part 1
    let connection_limit = if positions.len() == 20 {10} else {1000};
    // Construct a list of groups, one per position
    let mut groups:Vec<HashSet<usize>> = Vec::new();
    for i in 0..positions.len() {
        groups.push(HashSet::from([i]));

    }
    // Merge groups by adding connections in shortest connection order up to 
    // the part 1 limit
    for i in 0..connection_limit {
        let (_,a,b) = dists[i];
        merge(&mut groups, a, b);
    }
    // Get the sizes of the 3 largest groups
    let mut lengths = Vec::new();
    for i in 0..groups.len() {
        lengths.push(groups[i].len());
    }
    lengths.sort();
    lengths.reverse();
    for i in 0..3 {
        part1 *= lengths[i];
    }
    // Continue merging groups until all are connected and record the product
    // of the x coordinates for the final connected positions
    for i in connection_limit..dists.len() {
        let (_, a,b) = dists[i];
        merge(&mut groups, a, b);
        if groups.len() == 1 {
            part2 = positions[a].x * positions[b].x;
            break;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
