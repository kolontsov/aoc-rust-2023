pub mod day01;

pub type DayFn = fn(String);

pub fn nop(_inp: String) {}

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, nop),
        _ => {
            println!("Unknown day: {}", day);
            return (nop, nop);
        }
    };
}