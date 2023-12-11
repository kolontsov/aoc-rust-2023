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


pub fn part1(input: String) -> u64 {
    let mut map = Map::from_str(&input);
    map.show();
    for (x, y) in map.find_iter('#') {
        println!("Found S at ({}, {})", x, y);
    }
    map.width as u64
}
