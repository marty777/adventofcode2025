use crate::util;
use crate::util::Vec2;
use std::collections::{HashSet,HashMap};
use std::cmp::{min,max};

/// Ray cast from left to `pos` and count crossings of polygon horizontal line
/// segments to determine if the coordinate is inside or outside the polygon 
/// perimeter
fn coord_in_polygon(pos:Vec2, 
                    red_tiles:&HashSet<Vec2>, 
                    positions_by_x:&HashMap<isize, Vec<isize>>, 
                    positions_by_y:&HashMap<isize, Vec<isize>>, 
                    ordered_x_coords:&Vec<isize>) -> bool {
    // If the coord is a red tile, it is in the polygon
    if red_tiles.contains(&pos) { return true; }
    // If the coord is on a horizontal perimeter segment it is in the polygon.
    if positions_by_y.contains_key(&pos.y) {
        let min_x = positions_by_y.get(&pos.y).unwrap()[0];
        let max_x = positions_by_y.get(&pos.y).unwrap()[1];
        if pos.x > min_x && pos.x < max_x {
            return true;
        }
    }
    // Otherwise count crossings. An even number of crossings means the point 
    // is outside the polygon, an odd number means the point is inside.
    let y = pos.y;
    let mut crossings = 0;
    for x_index in 0..ordered_x_coords.len() {
        let x = ordered_x_coords[x_index];
        if x > pos.x {
            break;
        }
        let min_y = positions_by_x.get(&x).unwrap()[0];
        let max_y = positions_by_x.get(&x).unwrap()[1];
        // There's an issue with overcounting perimeter crossings if the ray
        // passes through a horizontal perimeter segment. Excluding either the 
        // upper or lower line segment endpoint from being counted handles that
        if y >= min_y && y < max_y {
            // If the position is on a polygon perimeter segment, it is in the 
            // polygon.
            if x == pos.x {
                return true;
            }
            crossings += 1;
        }
    };
    return (crossings % 2) == 1;
}
/// Determine if a perimeter line segment for a rectangle with endpoints `pos1`
/// and `pos2`  is entirely inside the polygon by testing for crossings of 
/// horizontal or vertical polygon perimeter segments.
fn perimeter_segment_in_polygon(pos1:Vec2, 
                                pos2:Vec2, 
                                red_tiles:&HashSet<Vec2>, 
                                positions_by_x:&HashMap<isize, Vec<isize>>, 
                                positions_by_y:&HashMap<isize, Vec<isize>>, 
                                ordered_x_coords:&Vec<isize>, 
                                ordered_y_coords:&Vec<isize>) -> bool {
    // If this isn't a horizontal or vertical segment, it's unhandled.
    assert!(pos1.x == pos2.x || pos1.y == pos2.y, "Unhandled line segment coord {} {}: {} not veritcal or horizontal", pos1, pos2, pos1.x);
    // Only permit testing of coordinates that share a horizontal or vertical
    // coordinate with a tile position
    assert!(positions_by_x.contains_key(&pos1.x), "Unhandled line segment coord {} {}: {} not in x", pos1, pos2, pos1.x);
    assert!(positions_by_x.contains_key(&pos2.x), "Unhandled line segment coord {} {}: {} not in x", pos1, pos2, pos2.x);
    assert!(positions_by_y.contains_key(&pos1.y), "Unhandled line segment coord {} {}: {} not in y", pos1, pos2, pos1.y);
    assert!(positions_by_y.contains_key(&pos2.y), "Unhandled line segment coord {} {}: {} not in y", pos1, pos2, pos2.y);
    // An unstated simplification in the input - there are either 0 or 2 tiles
    // in any row and column. If pos1 and pos2 are both red tiles and share a 
    // row or column, the line segment is on the perimeter of the polygon and 
    // is therefore inside the polygon.
    if red_tiles.contains(&pos1) && red_tiles.contains(&pos2) {
        return true;
    }
    // If either point is not inside the polygon, the line segment leaves the 
    // polygon
    if !coord_in_polygon(pos1, red_tiles, positions_by_x, positions_by_y, ordered_x_coords) ||
        !coord_in_polygon(pos2, red_tiles, positions_by_x, positions_by_y, ordered_x_coords) {
        return false;
    }
    // Cast a ray from one point to the other horizontally or vertically.
    // If the ray crosses a polygon perimeter segment, the line segment is not
    // fully contained in the polygon.
    let cast_horizontal = pos1.x != pos2.x;
    if cast_horizontal {
        let min_x = min(pos1.x, pos2.x);
        let max_x = max(pos1.x, pos2.x);
        let y = pos1.y;
        let x_start_index = ordered_x_coords.binary_search(&min_x).unwrap();
        let x_end_index = ordered_x_coords.binary_search(&max_x).unwrap();
        for x_index in x_start_index..=x_end_index {
            let x = ordered_x_coords[x_index];
            let min_y = positions_by_x.get(&x).unwrap()[0];
            let max_y = positions_by_x.get(&x).unwrap()[1];
            // If the line segment intersects a vertical perimeter line 
            // segment of the polygon and doesn't start or stop at that 
            // perimeter, it leaves the polygon.  
            if y > min_y && y < max_y && x != min_x && x != max_x {
                return false;
            }
        }
    }
    else {
        let min_y = min(pos1.y, pos2.y);
        let max_y = max(pos1.y, pos2.y);
        let x = pos1.x;
        let y_start_index = ordered_y_coords.binary_search(&min_y).unwrap();
        let y_end_index = ordered_y_coords.binary_search(&max_y).unwrap();
        for y_index in y_start_index..=y_end_index {
            let y = ordered_y_coords[y_index];
            let min_x = positions_by_y.get(&y).unwrap()[0];
            let max_x = positions_by_y.get(&y).unwrap()[1];
            // If the line segment intersects a horizontal perimeter line 
            // segment of the polygon and doesn't start or stop at that 
            // perimeter, it leaves the polygon. 
            if x > min_x && x < max_x && y != min_y && y != max_y {
                return false;
            }
        }
    }
    return true;
}
/// Insert `val` into `vec` in sorted order.
fn insert_sorted_isize(vec:&mut Vec<isize>, val:isize) {
    match vec.binary_search(&val) {
        Ok(_) => {}, // val already present, do nothing
        Err(index) => vec.insert(index, val)
    };
}
/// Area of a rectangle with corners `pos1`,`pos2`
fn area(pos1:&Vec2, pos2:&Vec2) -> isize {
    return ((pos1.x - pos2.x).abs() + 1) * ((pos1.y - pos2.y).abs() + 1);
}
/// Print the solutions to day 9 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let mut part2 = 0;

    // List of tile positions for consistant iteration
    let mut red_tiles = Vec::new();
    // Set of tile positions for quick queries
    let mut red_tiles_set = HashSet::new();
    // Sorted lists of y coordinates of red lights indexed by x coordinate
    let mut positions_by_x:HashMap<isize, Vec<isize>> = HashMap::new();
    // Sorted lists of x coordinates of red lights indexed by y coordinate
    let mut positions_by_y:HashMap<isize, Vec<isize>> = HashMap::new();
    
    // Read the input and populate the collections of tile position information
    let numbers = util::ints_in_strings(lines);
    for i in 0..numbers.len() {
        let red_tile = util::Vec2::new(numbers[i][0], numbers[i][1]);
        red_tiles.push(red_tile);
        red_tiles_set.insert(red_tile);
        // For each distinct horizontal coordinate, build a lookup of the 
        // sorted vertical coordinates.
        if positions_by_x.contains_key(&red_tile.x) {
            insert_sorted_isize(positions_by_x.get_mut(&red_tile.x).unwrap(), red_tile.y);
        }
        else {
            positions_by_x.insert(red_tile.x, vec![red_tile.y]);
        }
        // For each distinct vertical coordinate, build a lookup of the 
        // sorted horizontal coordinates
        if positions_by_y.contains_key(&red_tile.y) {
            insert_sorted_isize(positions_by_y.get_mut(&red_tile.y).unwrap(), red_tile.x);
        }
        else {
            positions_by_y.insert(red_tile.y, vec![red_tile.x]);
        }
    }

    // Get all horizontal and vertical coordinates of tiles in sorted orders
    let mut ordered_x_coords:Vec<isize> = positions_by_x.keys().cloned().collect();
    ordered_x_coords.sort();
    let mut ordered_y_coords:Vec<isize> = positions_by_y.keys().cloned().collect();
    ordered_y_coords.sort();

    // Find the area of the rectangle for each pair of tiles and test if it's
    // entirely contained in the polygon. Record the greatest overall area and
    // the greatest area contained in the polygon.
    for i in 0..red_tiles.len() {
        for j in i+1..red_tiles.len() {
            if i == j {
                continue;
            }
            let area = area(&red_tiles[i], &red_tiles[j]);
            if area > part1 {
                part1 = area;
            }
            if area > part2 {
                // Test if each perimeter line segment of the rectangle is 
                // entirely inside the polygon using a ray-casting approach. If 
                // so, the rectangle is within the polygon.
                let min_x = min(red_tiles[i].x, red_tiles[j].x);
                let max_x = max(red_tiles[i].x, red_tiles[j].x);
                let min_y = min(red_tiles[i].y, red_tiles[j].y);
                let max_y = max(red_tiles[i].y, red_tiles[j].y);
                if perimeter_segment_in_polygon(Vec2::new(min_x, min_y), Vec2::new(min_x, max_y), &red_tiles_set, &positions_by_x, &positions_by_y, &ordered_x_coords, &ordered_y_coords) &&
                    perimeter_segment_in_polygon(Vec2::new(min_x, min_y), Vec2::new(max_x, min_y), &red_tiles_set, &positions_by_x, &positions_by_y, &ordered_x_coords, &ordered_y_coords) &&
                    perimeter_segment_in_polygon(Vec2::new(max_x, min_y), Vec2::new(max_x, max_y), &red_tiles_set, &positions_by_x, &positions_by_y, &ordered_x_coords, &ordered_y_coords) &&
                    perimeter_segment_in_polygon(Vec2::new(min_x, max_y), Vec2::new(max_x, max_y), &red_tiles_set, &positions_by_x, &positions_by_y, &ordered_x_coords, &ordered_y_coords) {
                    part2 = area; 
                }
            }            
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
