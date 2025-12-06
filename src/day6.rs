use crate::util;

/// Print the solutions to day 6 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let mut part2 = 0;
    // Part 1 - perform the operations with numbers read left-to-right.
    // Parse the numbers in each row.
    let mut numbers:Vec<Vec<isize>> = Vec::new();
    for i in 0..lines.len() - 1 {
        numbers.push(util::ints_in_string(&lines[i]));
    }
    // Extract the operators
    let operators:Vec<&str> = lines[lines.len() - 1].split_whitespace().collect();
    // Perform each operation in sequence and sum the result into part1.
    for i in 0..operators.len() {
        if operators[i] == "+" {
            let mut sum = 0;
            for j in 0..numbers.len() {
                sum += numbers[j][i];
            }
            part1 += sum;
        }
        else {
            let mut product = 1;
            for j in 0..numbers.len() {
                product *= numbers[j][i];
            }
            part1 += product;
        }
    }
    // Part 2 - perform the operations with the numbers read top-to-bottom.
    // Read all characters in the input into a grid.
    let grid = util::read_grid(lines);
    let width = grid[0].len();
    let height = grid.len();
    let operator_line = height-1;
    let mut operator_index = 0;
    loop {
        // Get width of operation by finding the index of the next operator 
        // or the end of the line.
        let mut op_width = width - operator_index;
        for i in (operator_index + 1)..width {
            if grid[operator_line][i] != ' ' {
                op_width = i - operator_index - 1;
                break;
            }
        }
        // Determine the operation type and initialize an accumulator. 
        // 1 for products, 0 for sums.
        let operator = grid[operator_line][operator_index];
        let mut accumulator = 0;
        if operator == '*' {
            accumulator = 1;
        }
        // Compose the number in each column and add/multiply with the 
        // accumulator.
        for col in (operator_index..operator_index + op_width).rev() {
            let mut num = 0;
            for row in 0..height-1 {
                if grid[row][col] != ' ' {
                    num *= 10;
                    num += grid[row][col].to_string().parse::<isize>().unwrap();
                }
            }
            if operator == '*' {
                accumulator *= num;
            }
            else {
                accumulator += num;
            }
        }        
        part2 += accumulator;
        // Move to the next operation or end
        operator_index += op_width + 1;
        if operator_index > width {
            break;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
