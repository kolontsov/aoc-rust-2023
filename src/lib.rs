pub mod day01;
pub mod day02;
pub mod day03;

pub type DayFn = fn(String);

pub fn nop(_inp: String) {}

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, nop),
        _ => {
            eprintln!("Unknown day: {}", day);
            return (nop, nop);
        }
    };
}