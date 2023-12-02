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

fn run_dayfn(dayfn: DayFn, input: Option<String>, test: Option<String>) {
    if test != None {
        println!("{}", "<test>".bright_black());
        dayfn(test.unwrap());
        println!("{}", "</test>\n".bright_black());
    }
    if input != None {
        println!("{}", "<input>".green());
        let ts = time::Instant::now();
        dayfn(input.unwrap());
        println!("{}", format!("</input> {}ms\n", ts.elapsed().as_millis()).green());
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
    
    if to_run.0 != nop {
        println!("{}", "==== Part 1 ====".cyan());
        run_dayfn(to_run.0, input.clone(), test.clone());
    }
    if to_run.1 != nop {
        println!("{}", "==== Part 2 ====".cyan());
        run_dayfn(to_run.1, input.clone(), test.clone());
    }
    println!("{}", "DONE".cyan());
}

