use core::panic;

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    let mut result: Vec<Vec<&str>> = Vec::new();
    let mut map: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            result.push(map);
            map = Vec::new();
        } else {
            map.push(line);
        }
    }
    if !map.is_empty() {
        result.push(map);
    }
    result
}

fn has_horizontal_reflection(map: &Vec<&str>, row: usize, tolerance: usize) -> bool {
    let mut up = row;
    let mut down = row+1;
    let mut err = 0;
    loop {
        err += map[up].chars().zip(map[down].chars()).filter(|&(a, b)| a != b).count();
        if err > tolerance {
            return false;
        }
        if up == 0 || down == map.len()-1 {
            break;
        };
        up -= 1;
        down += 1;
    }
    return err == tolerance
}

fn has_vertical_reflection(map: &Vec<&str>, col: usize, tolerance: usize) -> bool {
    let mut left = col;
    let mut right = col+1;
    let mut err = 0;
    loop {
        for line in map {
            if line.chars().nth(left).unwrap() != line.chars().nth(right).unwrap() {
                err += 1;
            }
        }
        if err > tolerance {
            return false;
        }
        if left == 0 || right == map[0].len()-1 {
            break;
        }
        left -= 1;
        right += 1;
    }
    return err==tolerance;
}

fn get_mirror_value(map: &Vec<&str>, tolerance: usize) -> usize {
    let max_row = map.len()-1;
    let max_col = map[0].len()-1;
    for row in 0..max_row {
        if has_horizontal_reflection(&map, row, tolerance) {
            return 100*(row+1);
        }
    }
    for col in 0..max_col {
        if has_vertical_reflection(&map, col, tolerance) {
            return col+1;
        }
    }
    panic!("No reflection found");
}

pub fn part1(input: String) -> u64 {
    let maps = parse_input(&input);
    maps.iter().map(|map| get_mirror_value(&map, 0)).sum::<usize>() as u64
}

pub fn part2(input: String) -> u64 {
    let maps = parse_input(&input);
    maps.iter().map(|map| get_mirror_value(&map, 1)).sum::<usize>() as u64
}
