pub mod util;
// Day modules
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use clap::{Arg, Command};
use std::time::{Instant};

const MAXDAY:usize = 6;

fn main() {
    
    let command = Command::new("adventofcode2025").max_term_width(80)
					.about("Advent of Code 2025 solutions")
                    .arg(Arg::new("day").help("Puzzle day to run").short('d').long("day").value_name("DAY").required(true))
                    .arg(Arg::new("input").help("Path to input file").short('i').long("input").value_name("PATH"));
	let args = command.get_matches();	
	let day:usize;
    match args.get_one::<String>("day") {
        Some(day_arg) => {
            match day_arg.parse::<usize>() {
                Ok(n) => {
                    day = n;
                    if day < 1 || day > MAXDAY {
                        println!("DAY must be in range 1 - {} ({} provided)", MAXDAY, n);
                        std::process::exit(2);
                    }
                },
                Err(_) => {
                    println!("Could not parse DAY argument '{}' as an integer.", day_arg);
                    std::process::exit(2);
                }
            }
        },
        None => {
            println!("Day argument is required");
            std::process::exit(2);
        }
    }
    let input_path:String;
    match args.get_one::<String>("input") {
        Some(input_arg) => {
            input_path = input_arg.clone();
        },
        None => {
            input_path = format!("./data/day{}/input.txt", day);
        }
    }
    println!(r#"
   ___     __              __         ___  _____        __      _  ___  ____
  / _ |___/ /  _____ ___  / /_  ___  / _/ / ___/__  ___/ /__   ( )|_  |/ __/
 / __ / _  / |/ / -_) _ \/ __/ / _ \/ _/ / /__/ _ \/ _  / -_)  |// __//__ \ 
/_/ |_\_,_/|___/\__/_//_/\__/  \___/_/   \___/\___/\_,_/\__/    /____/____/ "#);
    
    println!("\nDay:\t{}\nInput:\t{}\n", day, input_path);
            
    match util::read_input(&input_path) {
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(2);
        },
        Ok(input_lines) => {
            let start_instant = Instant::now();
            match day {
                1 => day1::run(&input_lines),
                2 => day2::run(&input_lines),
                3 => day3::run(&input_lines),
                4 => day4::run(&input_lines),
                5 => day5::run(&input_lines),
                6 => day6::run(&input_lines),
                _ => { println!("Day {} not matched", day)}
            }
            println!("\nTotal execution time: {:?}", start_instant.elapsed());
        }
    }
}
