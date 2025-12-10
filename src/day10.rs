
use crate::util;

struct Machine {
    pub indicator:Vec<isize>,
    pub buttons:Vec<Vec<isize>>,
    //pub joltages:Vec<isize>
}
impl Machine {
    pub fn try_buttons(&self, attempt:usize) -> bool {
        let mut state = vec![0;self.indicator.len()];
        let digits = util::base_n_digits(attempt as i128, 2, Some(self.buttons.len()));
        for i in 0..digits.len() {
            if digits[i] == 1 {
                for b in self.buttons[i as usize].iter() {
                    state[*b as usize] = (state[*b as usize] + 1) % 2;
                }
            }
        }
        let mut okay = true;
        for i in 0..self.indicator.len() {
            if self.indicator[i] != state[i] {
                okay = false;
                break;
            }
        }
        return okay;
    }
}

pub fn bits(attempt:usize) -> usize {
    let digits = util::base_n_digits(attempt as i128, 2, None);
    let mut count = 0;
    for d in digits {
        count += d;
    }
    return count as usize;
}

/// Print the part 1 solution to day 10 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let mut part1 = 0;
    let mut machines:Vec<Machine> = Vec::new();
    for i in 0..lines.len() {
        let parts:Vec<&str> = lines[i].split(" ").collect();
        let mut indicator:Vec<isize> = Vec::new();
        let mut buttons:Vec<Vec<isize>> = Vec::new();
        //let mut joltages:Vec<isize> = Vec::new();
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
                // '{' => {
                //     joltages = util::ints_in_string(&part.to_string());
                // },
                _ => {}
            }
        }
        let machine = Machine{indicator:indicator, buttons:buttons, /*joltages:joltages*/};
        machines.push(machine);
    }
    for i in 0..machines.len() {
        let mut best_solution = 0;
        // The dumb way
        let max = 1 << machines[i].buttons.len();
        for j in 1..max {
            if machines[i].try_buttons(j) {
                let bits = bits(j);
                if best_solution == 0 || bits < best_solution {
                    best_solution = bits;
                }
            }
        }
        part1 += best_solution;   
    }
    println!("Part 1: {}", part1);
}
