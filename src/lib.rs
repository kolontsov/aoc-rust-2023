pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

pub type DayFn = fn(String) -> u64;

pub fn nop(_inp: String) -> u64 {
    0
}

pub fn get_day(day: u32) -> (DayFn, DayFn, u64, u64) {
    return match day {
        1 => (day01::part1, day01::part2, 0, 0),
        2 => (day02::part1, day02::part2, 0, 0),
        3 => (day03::part1, day03::part2, 4361, 467835),
        4 => (day04::part1, day04::part2, 13, 30),
        5 => (day05::part1, day05::part2, 35, 0),
        6 => (day06::part1, day06::part2, 288, 71503),
        7 => (day07::part1, day07::part2, 6440, 5905),
        8 => (day08::part1, day08::part2, 6, 6), 
        9 => (day09::part1, day09::part2, 114, 2), 
        10 => (day10::part1, day10::part2, 70, 8),
        11 => (day11::part1, day11::part2, 374, 0),
        12 => (day12::part1, day12::part2, 21, 525152),
        13 => (day13::part1, day13::part2, 405, 400),
        _ => {
            eprintln!("Unknown day: {}", day);
            return (nop, nop, 0, 0);
        }
    };
}
