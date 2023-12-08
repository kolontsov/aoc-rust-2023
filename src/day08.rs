use std::collections::HashMap;
use regex::Regex;

pub fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let moves = lines.next().unwrap().chars().collect();
    assert!(lines.next().unwrap().is_empty());
    let mut rules = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in lines {
        let cap = re.captures_iter(line).next().unwrap();
        // println!("{} = ({}, {})", &cap[1], &cap[2], &cap[3]);
        rules.insert(cap[1].to_string(), (cap[2].to_string(), cap[3].to_string()));
    }
    (moves, rules)
}

pub fn part1(input: String) -> u64 {
    let (moves, rules) = parse(&input);
    let mut location = "AAA";
    let mut move_pos = 0;
    let mut total = 0;
    while location != "ZZZ" {
        match moves.get(move_pos).unwrap() {
            'L' => { location = rules.get(location).unwrap().0.as_str(); },
            'R' => { location = rules.get(location).unwrap().1.as_str(); },
            _ => { panic!("Unknown move: {}", moves[move_pos]); }
        }
        move_pos = (move_pos+1) % moves.len();
        total += 1;
    }
    total
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(numbers: &[u64]) -> u64 {
    numbers.iter().copied().reduce(|a, b| (a * b) / gcd(a, b)).unwrap_or(1)
}

pub fn part2(input: String) -> u64 {
    let (moves, rules) = parse(&input);
    let mut locations: Vec<String> = rules.keys()
        .filter(|loc| loc.ends_with('A'))
        .cloned()
        .collect();
    let mut move_pos = 0;
    let mut steps = 0;
    let mut periods = vec![0; locations.len()];
    while periods.iter().any(|p| *p == 0) {
        // println!("{:?}", locations);
        for i in 0..locations.len() {
            let loc = &locations[i];
            if loc.ends_with('Z') && periods[i] == 0 {
                periods[i] = steps;
            }
            match moves.get(move_pos).unwrap() {
                'L' => { locations[i] = rules.get(loc).unwrap().0.clone(); },
                'R' => { locations[i] = rules.get(loc).unwrap().1.clone(); },
                _ => { panic!("Unknown move: {}", moves[move_pos]); }
            }
        }
        move_pos = (move_pos+1) % moves.len();
        steps += 1;
    }
    println!("{:?}", periods);
    lcm(&periods)
}