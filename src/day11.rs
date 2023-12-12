use colored::Colorize;

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy)]
struct Tile {
    ch: char,
}

type XY = (usize, usize);

#[allow(dead_code)]
impl Map {
    fn new(width: usize, height: usize) -> Map {
        let tiles = vec![vec![Tile { ch: '.' }; width]; height];
        Map { width, height, tiles }
    }
    fn from_str(input: &str) -> Map {
        let tiles: Vec<Vec<Tile>> = input.lines().map(|line| line.chars().map(|ch| Tile { ch }).collect()).collect();
        let height = tiles.len();
        let width = tiles.first().map_or(0, |row| row.len());
        Map { width, height, tiles }
    }
    fn get_row(&self, y: usize) -> Vec<Tile> {
        self.tiles[y].clone()
    }
    fn get_column(&self, x: usize) -> Vec<Tile> {
        let mut result = Vec::new();
        for row in &self.tiles {
            result.push(row[x]);
        }
        result
    }
    fn insert_row(&mut self, y: usize, row: Vec<Tile>) {
        if row.len() != self.width || y > self.height {
            panic!("Invalid dimensions for insert_row");
        }
        self.tiles.insert(y, row);
        self.height += 1;
    }
    fn insert_column(&mut self, x: usize, column: Vec<Tile>) {
        if column.len() != self.height || x > self.width {
            panic!("Invalid dimensions for insert_column");
        }
        for (i, row) in self.tiles.iter_mut().enumerate() {
            row.insert(x, column[i]);
        }
        self.width += 1;
    }
    fn remove_column(&mut self, x: usize) {
        if x >= self.width {
            panic!("Invalid column index for remove_column");
        }
        for row in &mut self.tiles {
            row.remove(x);
        }
        self.width -= 1;
    }
    fn remove_row(&mut self, y: usize) {
        if y >= self.height {
            panic!("Invalid row index for remove_row");
        }
        self.tiles.remove(y);
        self.height -= 1;
    }
    fn show(&self) {
        print!(" ");
        for x in 0..self.width { print!("{}", (x % 10).to_string().cyan()); }
        print!("\n");
        for (y, row) in self.tiles.iter().enumerate() {
            print!("{}", (y % 10).to_string().cyan());
            for tile in row {
                print!("{}", tile.ch);
            }
            println!();
        }
        println!();
    }
    fn find_iter<'a>(&'a self, ch: char) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.tiles.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().filter_map(move |(x, tile)| {
                if tile.ch == ch {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
    }
}
#[warn(dead_code)]

fn expand_universe(map: &mut Map) {
    for y in (0..map.height).rev() {
        let row = map.get_row(y);
        if row.iter().all(|tile| tile.ch == '.') {
            map.insert_row(y, row)
        }
    };
    for x in (0..map.width).rev() {
        let column = map.get_column(x);
        if column.iter().all(|tile| tile.ch == '.') {
            map.insert_column(x, column)
        }
    };
}

fn unique_pairs<T: Clone>(items: Vec<T>) -> Vec<(T, T)> {
    let mut pairs = Vec::new();
    for (i, item1) in items.iter().enumerate() {
        for item2 in items.iter().skip(i + 1) {
            pairs.push((item1.clone(), item2.clone()));
        }
    }
    pairs
}

fn dist(&(x1, y1): &XY, &(x2, y2): &XY) -> u64 {
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as u64
}

pub fn part1(input: String) -> u64 {
    let mut map = Map::from_str(&input);
    expand_universe(&mut map);
    // map.show();
    let galaxies: Vec<XY> = map.find_iter('#').collect();
    // println!("Galaxies: {:?}", galaxies);
    let pairs: Vec<(XY, XY)> = unique_pairs(galaxies);
    // println!("Pairs: {:?}", pairs.len());
    pairs.iter().map(|(p1, p2)| dist(p1, p2)).sum()
}

pub fn part2(input: String) -> u64 {
    let map = Map::from_str(&input);
    let mut galaxies: Vec<XY> = map.find_iter('#').collect();
    
    let factor: usize = 999_999;
    let is_empty = |tiles: &[Tile]| tiles.iter().all(|tile| tile.ch == '.');
    
    for y in (0..map.height).rev() {
        if is_empty(&map.get_row(y)) {
            galaxies.iter_mut().filter(|p| p.1>y).for_each(|p| p.1 += factor);
        }
    };

    for x in (0..map.width).rev() {
        if is_empty(&map.get_column(x)) {
            galaxies.iter_mut().filter(|p| p.0>x).for_each(|p| p.0 += factor);
        }
    };

    unique_pairs(galaxies).iter().map(|(p1, p2)| dist(p1, p2)).sum()
}

