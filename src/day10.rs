use core::panic;
use colored::Colorize;

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    visited: Vec<Point>,
}
struct Tile {
    ch: char,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    on: bool
}
type Point = (usize, usize);

impl Map {
    fn from_str(input: &str) -> Map {
        let on = false;
        let tiles: Vec<Vec<Tile>> = input.lines().map(|line| {
            line.chars().map(|ch| {
                match ch {
                    'S' => Tile { ch, on, left: true,  right: true,  up: true,  down: true },
                    'F' => Tile { ch, on, left: false, right: true,  up: false, down: true },
                    '7' => Tile { ch, on, left: true,  right: false, up: false, down: true },
                    'L' => Tile { ch, on, left: false, right: true,  up: true,  down: false },
                    'J' => Tile { ch, on, left: true,  right: false, up: true,  down: false },
                    '-' => Tile { ch, on, left: true,  right: true,  up: false, down: false },
                    '|' => Tile { ch, on, left: false, right: false, up: true,  down: true },
                    '.' => Tile { ch, on, left: false, right: false, up: false, down: false },
                    _ => panic!("Unknown tile: {}", ch),
                }
            }).collect()
        }).collect();
        let height = tiles.len();
        let width = tiles.first().map_or(0, |row| row.len());
        Map { width, height, tiles, visited: Vec::new() }
    }
    fn visit(&mut self, pos: Point) {
        self.visited.push(pos);
        let (x, y) = pos;
        self.tiles[y][x].on = true;
    }
    fn is_visited(&self, pos: Point) -> bool {
        self.visited.contains(&pos)
    }
    fn get_moves(&self, pos: Point) -> Vec<Point> {
        let (x, y) = pos;
        let tile = &self.tiles[y][x];
        let mut result = Vec::new();
        if tile.left && x>0 && self.tiles[y][x-1].right {
            result.push((x-1, y));
        }
        if tile.right && x<self.width-1 && self.tiles[y][x+1].left {
            result.push((x+1, y));
        }
        if tile.up && y>0 && self.tiles[y - 1][x].down {
            result.push((x, y-1));
        }
        if tile.down && y<self.height-1 && self.tiles[y+1][x].up {
            result.push((x, y+1));
        }
        result
    }
    fn get_neighbors(&self, pos: Point) -> Vec<Point> {
        let (x, y) = pos;
        let mut result = Vec::new();
        if x>0 {
            result.push((x-1, y));
        }
        if x<self.width-1 {
            result.push((x+1, y));
        }
        if y>0 {
            result.push((x, y-1));
        }
        if y<self.height-1 {
            result.push((x, y+1));
        }
        result
    }

    fn _beautify(&self, input: char) -> char {
            match input {
                'S' => 'S',
                'F' => '┌',
                '7' => '┐',
                'L' => '└',
                'J' => '┘',
                '-' => '─',
                '|' => '│',
                '.' => '.',
                _ => input
            }
    }
    fn _show(&self) {
        for y in (2..self.height).step_by(2) {
            for x in (2..self.width).step_by(2) {
                let tile = &self.tiles[y][x];
                let s = self._beautify(tile.ch).to_string();
                
                print!("{}", if tile.on { 
                    s.green()
                } else { 
                    if tile.ch=='.' { "█".red() } else { s.black() }
                });
            }
            println!()
        }
    }   
}

fn find_animal(map: &Map) -> (usize, usize) {
    for (y, line) in map.tiles.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if tile.ch == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No animal found");
}

fn find_loop(map: &mut Map, pos: Point) -> u32 {
    let mut len = 1;
    let mut pos = pos;
    loop {
        map.visit(pos);
        let moves: Vec<Point> = map.get_moves(pos).iter().filter(|&&npos| !map.is_visited(npos)).copied().collect();
        if moves.len() == 0 {
            break;
        }
        pos = *moves.first().unwrap();
        len += 1;
    }
    len
}

pub fn part1(input: String) -> u64 {
    let mut map = Map::from_str(&input);
    let pos = find_animal(&map);
    let len = find_loop(&mut map, pos);
    (len/2) as u64
}

fn x2(input: &str) -> String {
    let mut result = String::new();
    let mut filler = String::new();
    for (i, line) in input.lines().enumerate() {
        if i==0 {
            filler = format!("{}\n", ".".repeat(line.len()*2+4));
            result.push_str(&filler);
            result.push_str(&filler);
        }
        let line = format!(".{}.", line);
        let mut cur_line = String::new();
        let mut next_line = String::new();
        for ch in line.chars() {
            cur_line.push(ch);
            cur_line.push(match ch {
                'F' => '-',
                'L' => '-',
                '-' => '-',
                'S' => 'S',
                _ => '.',
            });
            next_line.push(match ch {
                'F' => '|',
                '7' => '|',
                '|' => '|',
                'S' => 'S',
                _ => '.',
            });
            next_line.push('.');
        }
        result.push_str(&cur_line);
        result.push('\n');
        result.push_str(&next_line);
        result.push('\n');
    }
    result.push_str(&filler);
    result.push_str(&filler);
    result
}

fn flood_fill(map: &mut Map, pos: Point){
    let (x, y) = pos;
    if map.tiles[y][x].ch!='.' {
        return;
    }
    map.tiles[y][x].ch = '#';
    for new_pos in &map.get_neighbors(pos) {
        flood_fill(map, *new_pos);
    }
}

pub fn part2(input: String) -> u64 {
    let mut map = Map::from_str(&x2(&input));
    let pos = find_animal(&map);
    find_loop(&mut map, pos);
    for y in 0..map.height {
        for x in 0..map.width {
            if !map.tiles[y][x].on {
                map.tiles[y][x].ch = '.';
            }
        }
    }
    flood_fill(&mut map, (0, 0));
    // map._show();

    let mut painted = 0;
    for y in (2..map.height).step_by(2) {
        for x in (2..map.width).step_by(2) {
            if map.tiles[y][x].ch == '.' {
                painted += 1;
            }
        }
    }
    painted as u64

}
