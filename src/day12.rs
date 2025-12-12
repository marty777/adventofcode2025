use crate::util;
use util::Vec2;
use core::num;
use std::{collections::HashSet};

struct Shape {
    pub transforms:Vec<HashSet<Vec2>>,
    pub id:usize
}
impl Shape {
    pub fn new(section:&Vec<String>) -> Shape {
        let (id_str, _) = section[0].split_at(1);
        let id = id_str.parse::<i128>().unwrap();
        let mut base_cells = HashSet::new();
        for y in 1..section.len() {
            let chars = section[y].chars().collect::<Vec<char>>();
            for x in 0..chars.len() {
                if chars[x] == '#' {
                    base_cells.insert(Vec2::newu(x, y-1));
                }
            }
        }
        let mut transforms = Vec::new();
        for rotate in 0..4 {
            let mut cells = base_cells.clone();
            for reflect in 0..1 {
                
                for i in 0..rotate {
                    cells = rotate_clockwise(&cells);
                }
                for i in 0..reflect {
                    cells = reflect_x(&cells);
                }
            }
            transforms.push(cells);
        }
        return Shape{transforms:transforms, id:id as usize};
    }
}

struct Region {
    x:isize,
    y:isize,
    shape_counts:Vec<isize>
}

pub fn reflect_x(cells:&HashSet<Vec2>) -> HashSet<Vec2> {
    let mut reflected = HashSet::new();
    for cell in cells {
        reflected.insert(Vec2::new(-cell.x, cell.y));
    }
    return reflected;
}

pub fn rotate_clockwise(cells:&HashSet<Vec2>) -> HashSet<Vec2> {
    // R = cos -sin
   //     sin cos
    let cos = 0;
    let sin = 1;
    let mut rotated = HashSet::new();
    for cell in cells {
        rotated.insert(Vec2::new(cos * cell.x -sin * cell.y, sin * cell.x + cos * cell.y));
    }
    return rotated;
}

/// Print the solutions to day 12 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let sections = util::sections(lines);
    let mut shapes = Vec::new();
    for i in 0..sections.len() - 1 {
        shapes.push(Shape::new(&sections[i]));
    }
    let numbers = util::ints_in_strings(sections.last().unwrap());
    let mut regions = Vec::new();
    for i in 0..numbers.len() {
        regions.push(Region{x:numbers[i][0], y:numbers[i][1], shape_counts:vec![numbers[i][2],numbers[i][3],numbers[i][4],numbers[i][5],numbers[i][6],numbers[i][7]]});
    }
    // I did spend over an hour pursuing other approaches before thinking to 
    // try this
    // Only the regions that have a smaller area than the sum of the cells of
    // all the required present shapes are unable to fit all the presents.
    for i in 0..regions.len() {
        let region_area = (regions[i].x * regions[i].y) as usize;
        let mut total_shape_size = 0;
        for j in 0..6 {
            total_shape_size += regions[i].shape_counts[j] as usize * shapes[j].transforms[0].len();
        }
        if total_shape_size <= region_area {
            part1 += 1;
        }
    }
    println!("Part 1: {}", part1);
}
