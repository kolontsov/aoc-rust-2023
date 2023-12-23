use std::collections::HashSet;

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<char>>,
}

impl Map {
    fn from_str(input: &str) -> Map {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| {
            line.chars().collect()
        }).collect();
        let height = tiles.len();
        let width = tiles.first().map_or(0, |row| row.len());
        Map { width, height, tiles }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir { Left, Down, Right, Up }

impl Dir {
    fn all_possible () -> Vec<Dir> {
        vec![Dir::Left, Dir::Down, Dir::Right, Dir::Up]
    }
    fn from_char(c: char) -> Option<Dir> {
        match c {
            '<' => Some(Dir::Left),
            'v' => Some(Dir::Down),
            '>' => Some(Dir::Right),
            '^' => Some(Dir::Up),
            _ => None,
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn successors(&self, map: &Map) -> Vec<Pos> {
        let mut result = Vec::new();
        for dir in Dir::all_possible() {
            if let Some(slope) = Dir::from_char(map.tiles[self.y][self.x]) {
                if dir != slope {
                    continue;
                }
            }
            let (dx, dy) = dir.get_delta();
            let (nx, ny) = (self.x as i32 + dx, self.y as i32 + dy);
            if nx<0 || ny<0 || (nx as usize)==map.width || (ny as usize)==map.height {
                continue;
            }
            let (x, y) = (nx as usize, ny as usize);
            if map.tiles[y][x] == '#' {
                continue;
            }
            result.push(Pos{ x, y });
        }

        result
  }
  fn is_final(&self, map: &Map) -> bool {
        self.x == map.width-2 && self.y == map.height-1
  }
}

fn dfs<F, G>(start: Pos, successors: F, is_final: G) -> u64
where
    F: Fn(Pos) -> Vec<Pos>,
    G: Fn(&Pos) -> bool,
{
    let mut stack = vec![(start, 0, HashSet::new())];
    let mut max_len = 0;

    while let Some((pos, cur_len, mut visited)) = stack.pop() {
        if !visited.insert(pos) {
            continue;
        }
        if is_final(&pos) {
            max_len = max_len.max(cur_len);
            // println!("Found path of length {}", max_len);
        } else {
            for next_pos in successors(pos) {
                if !visited.contains(&next_pos) {
                    stack.push((next_pos, cur_len + 1, visited.clone()));
                }
            }
        }
    }
    max_len as u64
}

pub fn part1(input: String) -> u64 {
    let map = Map::from_str(&input);
    dfs(Pos{ x: 1, y: 0 },
        |p| p.successors(&map),
        |p| p.is_final(&map))
}

pub fn part2(_: String) -> u64 {
    0
}
