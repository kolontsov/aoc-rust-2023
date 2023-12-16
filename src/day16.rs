use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Dir {
    Left = 0,
    Down,
    Right,
    Up,
}
type XY = (usize, usize);
type Map = Vec<Vec<char>>;

#[derive(Clone)]
struct Beam {
    pos: XY,
    dir: Dir,
    stopped: bool
}

const DELTA: [(i32, i32); 4] = [ (-1, 0), (0, 1), (1, 0), (0, -1)];

impl Beam {
    fn new(pos: XY, dir: Dir) -> Beam {
        Beam { pos, dir, stopped: false }
    }
    fn next_pos(&self, map: &Map) -> Option<XY> {
        let (dx, dy) = DELTA[self.dir as usize];
        let (x, y) = self.pos;
        let (x, y) = (x as i32 + dx, y as i32 + dy);
        if x >= 0 && y >= 0 && (x as usize) < map.len() && (y as usize) < map[0].len() {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
    fn reflect(&mut self, ch: char) {
        self.dir = match (self.dir, ch) {
            (Dir::Left, '/') => Dir::Down,
            (Dir::Left, '\\') => Dir::Up,
            (Dir::Right, '/') => Dir::Up,
            (Dir::Right, '\\') => Dir::Down,
            (Dir::Up, '/') => Dir::Right,
            (Dir::Up, '\\') => Dir::Left,
            (Dir::Down, '/') => Dir::Left,
            (Dir::Down, '\\') => Dir::Right,
            _ => self.dir,
        };
    }
    fn split(&mut self, ch: char) -> Option<Vec<Beam>> {
        match (self.dir, ch) {
            (Dir::Up | Dir::Down, '-') => {
                self.stopped = true;
                Some(vec![Beam::new(self.pos, Dir::Left), Beam::new(self.pos, Dir::Right)])
            },
            (Dir::Left | Dir::Right, '|') => {
                self.stopped = true;
                Some(vec![Beam::new(self.pos, Dir::Up), Beam::new(self.pos, Dir::Down)])
            }
            _ => { None }
        }
    }
    fn move_forward(&mut self, map: &Map) {
        if let Some(new_pos) = self.next_pos(map) {
            self.pos = new_pos;
        } else {
            self.stopped = true;
        }
    }
}

fn parse_input(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn trace(map: &Map, start: &Beam) -> u64 {
    let mut beams: Vec<Beam> = vec![start.clone()];
    let mut energized: HashSet<XY> = HashSet::new();
    let mut visited: HashSet<(XY, Dir)> = HashSet::new();
    while beams.len() > 0 {
        let mut to_add: Vec<Beam> = Vec::new();
        
        for beam in beams.iter_mut() {
            let visited_key = (beam.pos, beam.dir);
            if visited.contains(&visited_key) {
                beam.stopped = true;
                continue;
            }
            visited.insert(visited_key);
            energized.insert(beam.pos);
            
            let ch = map[beam.pos.1][beam.pos.0];
            if let Some(new_beams) = beam.split(ch) {
                to_add.extend(new_beams);
            }
            beam.reflect(ch);
            beam.move_forward(&map);
        }
        beams = beams.into_iter().filter(|beam| !beam.stopped).collect();
        beams.extend(to_add);        
    }
    energized.len() as u64
}

pub fn part1(input: String) -> u64 {
    let map: Map = parse_input(&input);
    trace(&map, &Beam::new((0, 0), Dir::Right))
}

pub fn part2(input: String) -> u64 {
    let map: Map = parse_input(&input);
    let max_x = map[0].len()-1;
    let max_y = map.len()-1;
    let mut start: Vec<Beam> = Vec::new();
    for y in 0..=max_y {
        start.push(Beam::new((0, y), Dir::Right));
        start.push(Beam::new((max_x, y), Dir::Left));
    }
    for x in 0..=max_x {
        start.push(Beam::new((x, 0), Dir::Down));
        start.push(Beam::new((x, max_y), Dir::Up));
    }
    start.iter().map(|beam| trace(&map, &beam)).max().unwrap()
}
