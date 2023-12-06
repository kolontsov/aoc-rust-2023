pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

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
        6 => (day06::part1, day06::part2, 288, 0),
        _ => {
            eprintln!("Unknown day: {}", day);
            return (nop, nop, 0, 0);
        }
    };
}
