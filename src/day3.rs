
/// Recurse over the digit selections for the given number of digits. At each 
/// index position, find the largest possible leftmost number then recurse to
/// the next digit.
pub fn battery_recurse(digits:&Vec<usize>, max_digits: usize, indices:&mut Vec<usize>, index:usize) -> Result<usize, String> {
    // If all selections have been made, compose the completed joltage
    if index == max_digits {
        let mut accumulator = 0;
        for i in 0..max_digits{
            accumulator *= 10;
            accumulator += digits[indices[i]];
        }
        return Ok(accumulator);
    }
    // Determine the minimum and maximum battery indexes the selection can be 
    // made from. Selections can't be made farther left than the previous
    // selected battery, and can't be made so far to the right that there isn't
    // space available for the remaining battery selections.
    let mut min_digit_index = 0;
    if index > 0 {
        min_digit_index = indices[index-1] + 1;
    }
    let max_digit_index = digits.len() - (max_digits - index);
    // Starting with 9 and working down, find the leftmost instance of that
    // digit available and try recursing to subsequent digits. No other battery
    // selection with the same digit can possibly perform better. Return the
    // first result found while descending the digits
    for d in (1..=9).rev() {
        for i in min_digit_index..=max_digit_index {
            if digits[i] == d {
                indices[index] = i;
                match battery_recurse(digits, max_digits, indices, index + 1) {
                    Ok(n) => return Ok(n),
                    Err(_) => continue
                }
            }
        }
    }
    return Err("Not found".to_string());
}

/// Print the solutions to day 3 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let mut part2 = 0;
    // Storage for the selected indices during recursion
    let mut indices1:Vec<usize> = vec![0;2];
    let mut indices2 = vec![0;12];
    for i in 0..lines.len() {
        let digits:Vec<usize> = lines[i].chars().map(|x| (x.to_string().parse::<usize>().unwrap())).collect();
        part1 += battery_recurse(&digits, 2, &mut indices1, 0).unwrap();
        part2 += battery_recurse(&digits, 12, &mut indices2, 0).unwrap();
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
