use std::env;
use std::fs;
use std::time;

use aoc_rust::{get_day, nop};

fn usage() -> ! {
    eprintln!("Usage: {} <day>", env::args().nth(0).unwrap());
    std::process::exit(1);
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

    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(format!("day{:02}.txt", day));
    println!("Reading input from {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error reading input file").trim().to_string();

    let to_run = get_day(day);
    if to_run.0 != nop {
        println!("\n== Part 1 ==");
        let ts = time::Instant::now();
        to_run.0(input.clone());
        println!("Time elapsed: {}ms", ts.elapsed().as_millis());
    }
    if to_run.1 != nop {
        println!("\n== Part 2 ==");
        let ts = time::Instant::now();
        to_run.1(input.clone());
        println!("Time elapsed: {}ms", ts.elapsed().as_millis());
    }
    println!("\n== Done ==");
}

