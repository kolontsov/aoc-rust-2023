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

fn horizontal_mirror(map: &Vec<&str>, row: usize, tolerance: usize) -> Option<u32> {
    let mut u = row;
    let mut d = row+1;
    let mut err = 0;
    loop {
        for (c1, c2) in map[u].chars().zip(map[d].chars()) {
            if c1 != c2 {
                if err == tolerance { return None }
                err += 1;
            }
        }
        if u == 0 || d == map.len()-1 {
            break;
        };
        u -= 1;
        d += 1;
    }
    if err != tolerance {
        return None;
    }
    Some(row as u32)
}

fn vertical_mirror(map: &Vec<&str>, col: usize, tolerance: usize) -> Option<u32> {
    let mut l = col;
    let mut r = col+1;
    let mut err = 0;
    loop {
        for line in map {
            if line.chars().nth(l).unwrap() != line.chars().nth(r).unwrap() {
                if err == tolerance { return None }
                err += 1;
            }
        }
        if l == 0 || r == map[0].len()-1 {
            break;
        }
        l -= 1;
        r += 1;
    }
    if err!=tolerance {
        return None;
    }
    Some(col as u32)
}

fn get_mirror(map: &Vec<&str>, tolerance: usize) -> u32 {
    let max_row = map.len()-1;
    let max_col = map[0].len()-1;
    for row in 0..max_row {
        if let Some(horiz) = horizontal_mirror(&map, row, tolerance) {
            return 100*(horiz+1);
        }
    }
    for col in 0..max_col {
        if let Some(vert) = vertical_mirror(&map, col, tolerance) {
            return vert+1;
        }
    }
    panic!("No reflection found");
}

pub fn part1(input: String) -> u64 {
    let maps = parse_input(&input);
    maps.iter().map(|map| get_mirror(&map, 0)).sum::<u32>() as u64
}

pub fn part2(input: String) -> u64 {
    let maps = parse_input(&input);
    maps.iter().map(|map| get_mirror(&map, 1)).sum::<u32>() as u64
}
