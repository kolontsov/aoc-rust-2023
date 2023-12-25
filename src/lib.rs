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
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

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
        14 => (day14::part1, day14::part2, 136, 64),
        15 => (day15::part1, day15::part2, 1320, 0),
        16 => (day16::part1, day16::part2, 46, 51),
        17 => (day17::part1, day17::part2, 102, 94),
        18 => (day18::part1, day18::part2, 62, 952408144115),
        19 => (day19::part1, day19::part2, 19114, 167409079868000),
        20 => (day20::part1, day20::part2, 11687500, 0),
        21 => (day21::part1, day21::part2, 0, 0),
        22 => (day22::part1, day22::part2, 5, 7),
        23 => (day23::part1, day23::part2, 94, 154),
        24 => (day24::part1, day24::part2, 2, 0),
        25 => (day25::part1, nop, 0, 0),
        _ => {
            eprintln!("Unknown day: {}", day);
            return (nop, nop, 0, 0);
        }
    };
}
