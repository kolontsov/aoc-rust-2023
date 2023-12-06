fn parse_str(input: &str) -> Vec<u64> {
    input.split_once(":").unwrap().1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn parse_all(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    let times = parse_str(lines.next().unwrap());
    let distances = parse_str(lines.next().unwrap());
    times.into_iter().zip(distances.into_iter()).collect()
}

pub fn part1(input: String) -> u64 {
    let data = parse_all(&input);
    println!("Input: {:?}", data);
    let mut total = 1;
    for (time, distance) in data {
        let mut t = 0;
        for i in 1..time-1 {
            if (time-i)*i > distance {
                t += 1;
            }
        }
        total *= t;
    }

    total
}

pub fn part2(input: String) -> u64 {
    part1(input.replace(" ", ""))
}