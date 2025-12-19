
use crate::util::{self, abs_mod, Rational};
use std::cmp::{min, max};
use std::collections::HashMap;

#[derive(Debug)]
struct Machine {
    pub indicator:Vec<isize>,
    pub buttons:Vec<Vec<isize>>,
    pub joltages:Vec<isize>
}
impl Machine {
    /// Find either a unique solution to the system of linear equations for the
    /// indicators or joltages or the minimal one.
    pub fn rref_solve(&self, part1:bool) -> usize {
        let rows = self.indicator.len();
        let cols = self.buttons.len() + 1; 
        // build augmented matrix
        let mut augmented_matrix:Vec<Vec<Rational>> = Vec::new();
        for i in 0..rows {
            let mut row: Vec<Rational> = Vec::new();
            for j in 0..self.buttons.len() {
                if self.buttons[j].contains(&(i as isize)) {
                    row.push(Rational::new_int(1));
                }
                else {
                     row.push(Rational::new_int(0));
                }
            }
            if part1 {
                row.push(Rational::new_int(self.indicator[i]));
            }
            else {
                row.push(Rational::new_int(self.joltages[i]));
            }
            augmented_matrix.push(row);
        }
        // row reduce augmented matrix down
        for pivot_index in 0..cols-1 {
            if pivot_index >= rows {
                break;
            }
            // swap row to one with non-zero entry in pivot column if needed
            if augmented_matrix[pivot_index][pivot_index] == 0 {
                for r in pivot_index..augmented_matrix.len() {
                    if augmented_matrix[r][pivot_index] != 0 {
                        Machine::swap_rows(&mut augmented_matrix, pivot_index, r);
                        break;
                    }
                }
            }
            Machine::scale_row(&mut augmented_matrix, pivot_index);
            if augmented_matrix[pivot_index][pivot_index] != 1 {
                continue;
            }
            assert!(augmented_matrix[pivot_index][pivot_index] == 1, "Row reduction error: pivot position {} not 1 ({})", pivot_index, augmented_matrix[pivot_index][pivot_index]);
            Machine::row_replacement_down(&mut augmented_matrix, pivot_index, part1);
        }
        // row reduce augmented matrix up
        for pivot_index in 1..cols {
            if pivot_index >= rows {
                break;
            }
            if augmented_matrix[pivot_index][pivot_index] != 1 {
                continue;
            }
            Machine::row_replacement_up(&mut augmented_matrix, pivot_index, part1);
        }
        // scale each row to integers
        for r in 0..rows {
            Machine::re_scale_row(&mut augmented_matrix, r);
        }
        
        let mut free_variables = Vec::new();
        for col in 0..self.buttons.len() {
            let non_zero_entries = augmented_matrix.iter().filter(|&row| row[col] != 0).count();
            if non_zero_entries != 1 {
                free_variables.push(col);
            }
        }
        let free_variable_max = if part1 { 1 } else {*self.joltages.iter().max().unwrap() as usize };
        // If a unique solution, return the sum of the augmented column
        if free_variables.len() == 0 {
            return augmented_matrix.iter().map(|row| row.last().unwrap().num as usize).sum();
        }
        // Otherwise, try all combinations of free variables and return the minimal number of button presses
        let mut assignments = vec![0;free_variables.len()];
        let minimal_presses = self.try_variables(&augmented_matrix, &free_variables, free_variable_max, 0, &mut assignments, part1);
        assert!(!minimal_presses.is_none(), "Error on machine {:?} - no solution found", self);
        return minimal_presses.unwrap();
    }
    /// Recursively try all combinations of button presses up to `free_variable_max` on the free variables, and return the 
    /// minimal solution found where the number of presses for each button is non-negative
    fn try_variables(&self, matrix:&Vec<Vec<Rational>>, free_variables:&Vec<usize>, free_variable_max:usize, index:usize, assignments:&mut Vec<usize>, part1:bool) -> Option<usize> {
        if index == free_variables.len() {
            let mut button_sum = 0;
            let mut free_variable_map = HashMap::new();
            for i in 0..free_variables.len() {
                free_variable_map.insert(free_variables[i], i);
            }
            let mut outputs = vec![0 as isize;self.joltages.len()];
            for i in 0..self.buttons.len() {
                if free_variable_map.contains_key(&i) {
                    button_sum += assignments[*free_variable_map.get(&i).unwrap()];
                    for j in 0..self.buttons[i].len() {
                        if part1 {
                            outputs[self.buttons[i][j] as usize] = abs_mod(outputs[self.buttons[i][j] as usize] as i128 + assignments[*free_variable_map.get(&i).unwrap()] as i128, 2) as isize;
                        }
                        else {
                            outputs[self.buttons[i][j] as usize] += assignments[*free_variable_map.get(&i).unwrap()] as isize;
                        }
                    }
                    continue;
                }
                // subtract free variables from augmented column
                let mut presses = matrix[i].last().unwrap().clone();
                for j in 0..free_variables.len() {
                    presses -= matrix[i][free_variables[j]] * Rational::new_int(assignments[j] as isize);
                }
                // If the pivot entry isn't 1, divide the result
                if !part1 {
                    presses /= matrix[i][i];
                }
                // Presses for each button must be non-negative and non-fractional
                if presses.denom != 1 {
                    return None;
                }
                if part1 {
                    presses.num = abs_mod(presses.num as i128, 2) as isize;
                }
                if presses < 0 || presses.denom != 1 {
                    //println!("presses {} on button {} for assignments {:?}", presses, i, assignments);
                    return None;
                }
                assert!(presses.denom == 1, "Error on try_variables: presses {}", presses);
                button_sum += presses.num as usize;
                for j in 0..self.buttons[i].len() {
                    if part1 {
                        outputs[self.buttons[i][j] as usize] = (outputs[self.buttons[i][j] as usize] + presses.num) % 2;
                    }
                    else {
                        outputs[self.buttons[i][j] as usize] += presses.num;
                    }
                }
            }
            if part1 {
                if outputs != self.indicator {
                    return None;
                }
            }
            else {
                if outputs != self.joltages {
                    return None;
                }
            }
            return Some(button_sum);
        }
        let mut min_button_presses = 0;
        for i in 0..=free_variable_max {
            assignments[index] = i;
            let result = self.try_variables(matrix, free_variables, free_variable_max, index + 1, assignments, part1);
            match result {
                Some(button_presses) => {
                    if min_button_presses == 0 || button_presses < min_button_presses {
                        min_button_presses = button_presses;
                    }
                },
                None => {}
            }
        }
        if min_button_presses == 0 {
            return None;
        }
        return Some(min_button_presses);
    }
    /// Swap rows `a`, `b` in the matrix
    fn swap_rows(matrix:&mut Vec<Vec<Rational>>, a:usize, b:usize) {
        assert!(a < matrix.len(), "Swap rows: row a {} > matrix rows {} ", a, matrix.len());
        assert!(b < matrix.len(), "Swap rows: row b {} > matrix rows {} ", b, matrix.len());
        let first_row_index = min(a,b);
        let second_row_index = max(a,b);
        let second_row = matrix.remove(second_row_index);
        let first_row = matrix.remove(first_row_index);
        matrix.insert(first_row_index, second_row);
        matrix.insert(second_row_index, first_row);
    }
    /// Scale row so that pivot entry is 1
    fn scale_row(matrix:&mut Vec<Vec<Rational>>, pivot_index:usize) {
        if matrix[pivot_index][pivot_index] == 1 || matrix[pivot_index][pivot_index] == 0 {
            return;
        }
        let divisor = matrix[pivot_index][pivot_index];
        for c in 0..matrix[pivot_index].len() {
            matrix[pivot_index][c] /= divisor;
        }
    }
    /// Scale row at `row_index` so that denominator in all entries is 1
    /// and any common gcd has been divided out
    fn re_scale_row(matrix:&mut Vec<Vec<Rational>>, row_index:usize) {
        let mut coefficient = 1;
        for i in 0..matrix[row_index].len() {
            if matrix[row_index][i] != 0  && coefficient % matrix[row_index][i].denom != 0 {
                coefficient *= matrix[row_index][i].denom;
            }
        }
        let mut non_zeros = Vec::new();
        for i in 0..matrix[row_index].len() {
            matrix[row_index][i] *= coefficient;
            if matrix[row_index][i] != 0 {
                non_zeros.push(matrix[row_index][i].num as i128);
            }
        }
        let divisor = util::gcd_list(&non_zeros) as isize;
        for i in 0..matrix[row_index].len() {
            matrix[row_index][i] /= divisor;
        }
    }
    /// Perform subtraction on rows following the pivot row such that the entry 
    /// in the pivot column for each is zero. The pivot row/column entry must be 1
    fn row_replacement_down(matrix:&mut Vec<Vec<Rational>>, pivot_index:usize, part1:bool) {
        assert!(matrix[pivot_index][pivot_index] == 1, "Row replacement: pivot entry not 1 ({})", matrix[pivot_index][pivot_index]);
        for r in pivot_index + 1..matrix.len() {
            let coefficient = -matrix[r][pivot_index];
            if coefficient == 0 {
                continue;
            }
            for c in 0..matrix[r].len() {
                if part1 {
                    matrix[r][c] =  Rational::new_int(util::abs_mod((matrix[r][c] + coefficient*matrix[pivot_index][c]).num as i128, 2) as isize);
                }
                else {
                    matrix[r][c] = matrix[r][c] + coefficient*matrix[pivot_index][c];
                }
            }
        }
    }
    /// Perform subtraction on rows following the pivot row such that the entry 
    /// in the pivot column for each is zero. The pivot row/column entry must be 1
    fn row_replacement_up(matrix:&mut Vec<Vec<Rational>>, pivot_index:usize, part1:bool) {
        assert!(matrix[pivot_index][pivot_index] == 1, "Row replacement: pivot entry not 1 ({})", matrix[pivot_index][pivot_index]);
        for r in 0..pivot_index {
            let coefficient = -matrix[r][pivot_index];
            if coefficient == 0 {
                continue;
            }
            for c in 0..matrix[r].len() {
                if part1 {
                    matrix[r][c] = Rational::new_int(util::abs_mod((matrix[r][c] + coefficient*matrix[pivot_index][c]).num as i128, 2) as isize);
                }
                else {
                    matrix[r][c] = matrix[r][c] + coefficient*matrix[pivot_index][c];
                }
            }
        }
    }
}

/// Print the part 1 solution to day 10 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut machines:Vec<Machine> = Vec::new();
    for i in 0..lines.len() {
        let parts:Vec<&str> = lines[i].split(" ").collect();
        let mut indicator:Vec<isize> = Vec::new();
        let mut buttons:Vec<Vec<isize>> = Vec::new();
        let mut joltages:Vec<isize> = Vec::new();
        for part in parts {
            match part.chars().nth(0).unwrap() {
                '[' => {
                    //indicator = &part[1..(part.len() - 1)];
                    for i in 1..part.len() - 1 {
                        if part.chars().nth(i).unwrap() == '#' {
                            indicator.push(1);
                        }
                        else {
                            indicator.push(0);
                        }
                    }
                },
                '(' => {
                    buttons.push(util::ints_in_string(&part.to_string()));
                },
                '{' => {
                    joltages = util::ints_in_string(&part.to_string());
                },
                _ => {}
            }
        }
        let machine = Machine{indicator:indicator, buttons:buttons, joltages:joltages};
        machines.push(machine);
    }
    for i in 0..machines.len() {
        let part1_presses = machines[i].rref_solve(true);
        part1 += part1_presses;
    }
    println!("Part 1: {}", part1);
    for i in 0..machines.len() {
        let part2_presses = machines[i].rref_solve(false);
        part2 += part2_presses;
    }
    println!("Part 2: {}", part2);
}
