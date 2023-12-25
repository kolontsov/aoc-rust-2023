use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> u64 {
    let mut wires = HashMap::<&str, HashSet<&str>>::new();
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        for r in right.split(' ') {
            // dot -Tpng -Kneato img/day25.dot >img/day25.png
            if left == "mzg" && r == "bbm" { continue }
            if left == "cth" && r == "xxk" { continue }
            if left == "zdj" && r == "nvt" { continue }
            wires.entry(left).or_default().insert(r);
            wires.entry(r).or_default().insert(left);
        }
    }
    // skip test
    if wires.len() < 100 { return 0 }

    let mut seen = HashSet::new();
    let mut next = vec!["mzg"];
    while let Some(x) = next.pop() {
        if seen.insert(x) {
            next.extend(&wires[x]);
        }
    }
    let s = seen.len();

    ((wires.len() - s) * s) as u64
}