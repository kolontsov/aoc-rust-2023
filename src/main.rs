use std::env;
use std::fs;
use std::time;

use colored::Colorize;
use aoc_rust::{DayFn, get_day, nop};

fn usage() -> ! {
    eprintln!("Usage: {} <day>", env::args().nth(0).unwrap());
    std::process::exit(1);
}

fn get_input(filename: &str) -> Option<String> {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(filename);
    fs::read_to_string(filename).ok()
}

fn run_dayfn(dayfn: DayFn, input: Option<String>, test: Option<String>, test_res: u64) {
    if test != None {
        let res = dayfn(test.unwrap());
        println!("{} {} {}", "<test>".bright_black(), res, "</test>".bright_black());
        if test_res > 0 && res != test_res {
            eprintln!("{}: {} was expected", "Test failed".red(), res);
            std::process::exit(1);
        }
    }
    if input != None {
        let ts = time::Instant::now();
        let res = dayfn(input.unwrap());
        println!("{} {} {}", "<input>".green(), res, format!("</input> ({}ms)", ts.elapsed().as_millis()).green());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day;

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        usage();
    }
    
    day = day.trim().to_string();
    let day: u32 = day.parse().unwrap_or_else(|_| {
        eprintln!("Invalid day: {}", day);
        usage();
    });

    let to_run = get_day(day);
    let input = get_input(&format!("day{:02}.txt", day));
    let test = get_input(&format!("day{:02}_test.txt", day));
    
    if input == None && test == None {
        eprintln!("No input or test file found for day {}", day);
        std::process::exit(1);
    }

    if to_run.0 != nop {
        println!("{}", "\n==== Part 1 ====".cyan());
        run_dayfn(to_run.0, input.clone(), test.clone(), to_run.2);
    }
    if to_run.1 != nop {
        println!("{}", "\n==== Part 2 ====".cyan());
        run_dayfn(to_run.1, input.clone(), test.clone(), to_run.3);
    }
    println!("{}", "\nDONE".cyan());
}

