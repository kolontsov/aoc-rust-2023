use regex::Regex;

#[derive(Debug)]
enum Dir { Left, Up, Right, Down }
impl Dir {
    fn from_str(str: &str) -> Dir {
        match str {
            "R"|"0" => Dir::Right,
            "D"|"1" => Dir::Down,
            "L"|"2" => Dir::Left,
            "U"|"3" => Dir::Up,
            _ => panic!("Invalid direction: {}", str),
        }
    }
}

#[derive(Debug)]
struct Instruction{
    dir: Dir,
    steps: i64,
}

impl Instruction  {
    fn from_str(str: &str) -> Self {
        let re = Regex::new(r"(\w+) (\w+) \(#(\w+)\)").unwrap();
        let cap = re.captures_iter(str).next().expect("Invalid instruction");
        Instruction {
            dir: Dir::from_str(&cap[1]),
            steps: cap[2].parse::<i64>().expect("Wrong steps number"),
        }
    }
    fn from_str2(str: &str) -> Self {
        let re = Regex::new(r"\w+ \w+ \(#(\w+)\)").unwrap();
        let cap = re.captures_iter(str).next().expect("Invalid instruction");
        let cap_str = &cap[1];
        Instruction {
            dir: Dir::from_str(&cap_str[5..6]),
            steps: <i64>::from_str_radix(&cap_str[0..5], 16).expect("Wrong steps number"),
        }
    }
}

fn get_area(instructions: &Vec<Instruction>) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut path = vec![(0, 0)];
    let mut perimeter = 0;
    for instr in instructions {
        match instr.dir {
            Dir::Left => x -= instr.steps,
            Dir::Up => y -= instr.steps,
            Dir::Right => x += instr.steps,
            Dir::Down => y += instr.steps,
        }
        path.push((x, y));
        perimeter += instr.steps;
    }
    // shoelace formulla
    let mut area = 0;
    for i in 0..path.len()-2 {
        let (x0, y0) = path[i];
        let (x1, y1) = path[i+1];
        area += x0 * y1 - x1 * y0;
    }
    
    area.abs() / 2 + perimeter / 2 + 1
}

pub fn part1(input: String) -> u64 {
    let instructions: Vec<Instruction> = input.lines().map(|line| Instruction::from_str(line)).collect();
    get_area(&instructions) as u64
}

pub fn part2(input: String) -> u64 {
    let instructions: Vec<Instruction> = input.lines().map(|line| Instruction::from_str2(line)).collect();
    get_area(&instructions) as u64
}