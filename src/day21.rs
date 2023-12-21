type Point = (usize, usize);
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<char>>,
    visited: Vec<Point>,
}

impl Map {
    fn from_str(input: &str) -> Map {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| { line.chars().collect()}).collect();
        let height = tiles.len();
        let width = tiles.first().map_or(0, |row| row.len());
        Map { width, height, tiles, visited: Vec::new() }
    }
    fn find_char(&self, needle: char) -> (usize, usize) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if *ch == needle {
                    return (x, y);
                }
            }
        }
        panic!("No start position");
    }
    fn get(&self, pos: Point) -> char {
        let (x, y) = pos;
        self.tiles[y][x]
    }
    fn is_visited(&self, pos: Point) -> bool {
        self.visited.contains(&pos)
    }
    fn visit(&mut self, pos: Point) {
        self.visited.push(pos);
    }
    fn get_moves(&self, pos: Point) -> Vec<Point> {
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
        result.iter().filter(|&pos|
            !self.is_visited(*pos) && self.get(*pos) == '.'
        ).copied().collect()
    }
    fn show(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                print!("{}", if self.is_visited((x, y)) { 'O' } else { *ch });
            }
            println!();
        }
    }
}

pub fn part1(input: String) -> u64 {
    let mut map = Map::from_str(&input);
    let mut cur_pos: Vec<Point> = vec![map.find_char('S')];
    map.tiles[cur_pos[0].1][cur_pos[0].0] = '.';
    for _ in 0..64 {
        let mut new_pos = Vec::new();
        for pos in cur_pos.drain(..) {
            for npos in map.get_moves(pos) {
                if !new_pos.contains(&npos) {
                    new_pos.push(npos);
                }
            }
        }
        cur_pos.append(&mut new_pos);
        // for (y, row) in map.tiles.iter().enumerate() {
        //     for (x, ch) in row.iter().enumerate() {
        //         print!("{}", if cur_pos.contains(&(x, y)) { 'O' } else { *ch });
        //     }
        //     println!();
        // }
        // println!("----------------");
    }
    cur_pos.len() as u64
}

pub fn part2(input: String) -> u64 {
    input.len() as u64
}
