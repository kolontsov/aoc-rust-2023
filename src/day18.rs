use regex::Regex;
use std::fmt;

#[derive(Debug)]
enum Dir { Left, Up, Right, Down }
impl Dir {
    fn from_str(str: &str) -> Dir {
        match str {
            "L" => Dir::Left,
            "U" => Dir::Up,
            "R" => Dir::Right,
            "D" => Dir::Down,
            _ => panic!("Invalid direction: {}", str),
        }
    }
}

struct RGB { r: u8, g: u8, b: u8 }
impl RGB {
    fn from_str(str: &str) -> Option<Self> {
        let r = u8::from_str_radix(&str[0..2], 16).ok()?;
        let g = u8::from_str_radix(&str[2..4], 16).ok()?;
        let b = u8::from_str_radix(&str[4..6], 16).ok()?;
        Some(RGB { r, g, b })
    }
}
impl fmt::Debug for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(R:0x{:02x}, G:0x{:02x}, B:0x{:02x})", self.r, self.g, self.b)
    }
}

#[derive(Debug)]
struct Instruction{
    dir: Dir,
    steps: u32,
    color: RGB,
}

impl Instruction  {
    fn from_str(str: &str) -> Self {
        let re = Regex::new(r"(\w+) (\w+) \(#(\w+)\)").unwrap();
        let cap = re.captures_iter(str).next().expect("Invalid instruction");
        Instruction {
            dir: Dir::from_str(&cap[1]),
            steps: cap[2].parse::<u32>().expect("Wrong steps number"),
            color: RGB::from_str(&cap[3]).expect("Wrong color"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| Instruction::from_str(line)).collect()
}

fn detect_box(instructions: &[Instruction]) -> (usize, usize, usize, usize) {
    let mut x = 0;
    let mut y = 0;
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for instr in instructions {
        match instr.dir {
            Dir::Left => x -= instr.steps as i32,
            Dir::Right => x += instr.steps as i32,
            Dir::Up => y -= instr.steps as i32,
            Dir::Down => y += instr.steps as i32,
        }
        if x < min_x { min_x = x; }
        if y < min_y { min_y = y; }
        if x > max_x { max_x = x; }
        if y > max_y { max_y = y; }
    }
    let start_x = -min_x as usize;
    let start_y = -min_y as usize;
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    (start_x+1, start_y+1, width+2, height+2)
}

fn get_neighbors((x, y): (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x>0 { result.push((x-1, y)); }
    if x<width-1 { result.push((x+1, y)); }
    if y>0 { result.push((x, y-1)); }
    if y<height-1 { result.push((x, y+1)); }
    result
}

fn flood_fill(map: &mut Vec<Vec<char>>, (x, y): (usize, usize)){
    let width = map[0].len();
    let height = map.len();
    if map[y][x]!='.' {
        return;
    }
    map[y][x] = '*';
    for (nx, ny) in get_neighbors((x, y), width, height) {
        flood_fill(map, (nx, ny));
    }
}

pub fn part1(input: String) -> u64 {
    let instructions = parse_input(&input);
    let (mut x, mut y, width, height) = detect_box(&instructions);
    let mut grid = vec![vec!['.'; width]; height];
    for instr in instructions {
        for _ in 0..instr.steps {
            match instr.dir {
                Dir::Left => x -= 1,
                Dir::Up => y -= 1,
                Dir::Right => x += 1,
                Dir::Down => y += 1,
            }
            grid[y as usize][x as usize] = '#';
        }
    }
    flood_fill(&mut grid, (0, 0));
    // for line in &grid {
    //     println!("{}", line.iter().collect::<String>());
    // }
    let mut total: usize = 0;
    for line in &grid {
        for ch in line {
            if *ch == '*' {
                total += 1;
            }
        }
    }
    (width*height - total) as u64
}

pub fn part2(input: String) -> u64 {
    input.len() as u64
}
