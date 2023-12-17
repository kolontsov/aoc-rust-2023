use pathfinding::prelude::dijkstra;

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<usize>>,
}

impl Map {
    fn from_str(input: &str) -> Map {
        let tiles: Vec<Vec<usize>> = input.lines().map(|line| {
            line.chars().map(|c| c.to_digit(10).expect("Expected a digit") as usize).collect()
        }).collect();
        let height = tiles.len();
        let width = tiles.first().map_or(0, |row| row.len());
        Map { width, height, tiles }
    }
    fn _show(&self) {
        for line in &self.tiles {
            for digit in line {
                print!("{}", digit);
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    Left,
    Down,
    Right,
    Up,
}

impl Dir {
    fn is_opposite(&self, other: &Dir) -> bool {
        match (self, other) {
            (Dir::Left, Dir::Right) => true,
            (Dir::Right, Dir::Left) => true,
            (Dir::Up, Dir::Down) => true,
            (Dir::Down, Dir::Up) => true,
            _ => false,
        }
    }
    fn get_delta(&self) -> (i32, i32) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Up => (0, -1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
    dir: Dir,
    count: usize,
}

impl Pos {
    fn successors(&self, map: &Map, min: usize, max: usize) -> Vec<(Pos, usize)> {
        let mut result = Vec::new();
        for new_dir in &[Dir::Left, Dir::Down, Dir::Right, Dir::Up] {   
            if new_dir.is_opposite(&self.dir) {
                continue;
            }
            let (dx, dy) = new_dir.get_delta();
            let (nx, ny) = (self.x as i32 + dx, self.y as i32 + dy);
            if nx<0 || ny<0 || (nx as usize)==map.width || (ny as usize)==map.height {
                continue;
            }
            if *new_dir == self.dir && self.count == max {
                continue;
            }
            if *new_dir != self.dir && self.count < min {
                continue;
            }
            let (x, y) = (nx as usize, ny as usize);
            let new_count = if *new_dir == self.dir { self.count + 1 } else { 1 };
            result.push((Pos{ x, y, dir: *new_dir, count: new_count }, map.tiles[y][x]));
        }

        result
  }
  fn is_final(&self, map: &Map, min: usize) -> bool {
        self.x == map.width-1 && self.y == map.height-1 && self.count >= min
  }
}

fn find_path(map: &Map, min: usize, max: usize) -> u64 {
    let mut res: Vec<usize> = Vec::new();
    
    for dir in &[Dir::Right, Dir::Down] {
        let dist = dijkstra(&Pos{ x: 0, y: 0, dir: *dir, count: 0 },
            |p| p.successors(&map, min, max),
            |p| p.is_final(&map, min));
        match dist {
            Some((_, d)) => res.push(d),
            None => { }
        }
    }  
    *res.iter().min().unwrap() as u64
}

pub fn part1(input: String) -> u64 {
    let map = Map::from_str(&input);
    find_path(&map, 0, 3)
}

pub fn part2(input: String) -> u64 {
    let map = Map::from_str(&input);
    find_path(&map, 4, 10)
}
