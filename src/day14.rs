type Map = Vec<Vec<char>>;

fn parse_input(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn tilt_north(map: &mut Map) {
    for y in 1..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != 'O' {
                continue;
            }
            map[y][x] = '.';
            let mut y2 = y-1;
            while map[y2][x] =='.' && y2 > 0 {
                y2 -= 1;
            }
            if map[y2][x] == '.' {
                map[y2][x] = 'O';
            } else {
                map[y2+1][x] = 'O';
            }
        }
    }
}

fn rotate90(map: &mut Map) {
    let map2 = map.clone();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map[x][map2.len()-y-1] = map2[y][x];
        }
    }
}

fn calc_load(map: &Map) -> u32 {
    let mut total = 0;
    let max_row = map.len();
    for y in 0..max_row {
        for x in 0..map[y].len() {
            if map[y][x] == 'O' {
                total += max_row-y;
            }
        }
    }
    total as u32
}

pub fn part1(input: String) -> u64 {
    let mut map = parse_input(&input);
    tilt_north(&mut map);
    calc_load(&map) as u64
}

const MAX_CYCLES : usize = 1_000_000_000;

pub fn part2(input: String) -> u64 {
    let mut map = parse_input(&input);
    let mut history: Vec<[u32; 4]> = Vec::new();
    for _ in 0..MAX_CYCLES {
        let mut data = [0; 4];
        for i in 0..4 {
            tilt_north(&mut map);
            rotate90(&mut map);
            data[i] = calc_load(&map);
        }
        match history.iter().position(|&x| x == data) {
            None => history.push(data),
            Some(start) => {
                let loop_size = history.len() - start;
                let offset = start + (MAX_CYCLES-start-1) % loop_size;
                return history[offset][3] as u64;
            }
        }
    }
    unreachable!()
}