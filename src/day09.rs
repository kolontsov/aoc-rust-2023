pub fn parse(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| {
        line.split_whitespace().map(|num| { num.parse().unwrap() }).collect()
    }).collect()
}

pub fn build_history(numbers: &[i64], hist_fn: fn(&[i64]) -> i64) -> Vec<i64> {
    let mut history: Vec<i64> = Vec::new();
    let mut cur = numbers.to_vec();
    while cur.iter().any(|&n| n != 0) {
        history.push(hist_fn(&cur));
        cur = cur.windows(2).map(|win| win[1]-win[0]).collect();
    }
    history
}

pub fn part1(input: String) -> u64 {
    let numbers = parse(&input);
    let mut total = 0;
    for line in &numbers {
        let history = build_history(&line, |v| v[v.len()-1]);
        total += history.iter().sum::<i64>();
    }
    total as u64
}

pub fn part2(input: String) -> u64 {
    let numbers = parse(&input);
    let mut total = 0;
    for line in &numbers {
        let history = build_history(&line, |v| v[0]);
        let mut res = 0;
        for i in history.iter().rev() { res = i-res; }
        total += res;
    }
    total as u64
}