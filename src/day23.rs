use std::collections::{HashSet, HashMap};

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

    fn neighbors(&self, pos: &Pos, with_slopes: bool) -> Vec<Pos> {
        let mut result = Vec::new();
        for dir in Dir::all_possible() {
            if let Some(slope) = Dir::from_char(self.tiles[pos.y][pos.x]) {
                if with_slopes && dir != slope {
                    continue;
                }
            }
            let (dx, dy) = dir.get_delta();
            let (nx, ny) = (pos.x as i32 + dx, pos.y as i32 + dy);
            if nx<0 || ny<0 || (nx as usize)==self.width || (ny as usize)==self.height {
                continue;
            }
            let (x, y) = (nx as usize, ny as usize);
            if self.tiles[y][x] == '#' {
                continue;
            }
            result.push(Pos{ x, y});
        }
        result
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

type AdjacentsMap = HashMap<Pos, Vec<(Pos, usize)>>;
type VisitedSet = HashSet<Pos>;

fn dfs(graph: &AdjacentsMap, visited: &mut VisitedSet, pos: &Pos, end_pos: &Pos) -> Option<usize>
{
    if pos == end_pos {
        return Some(0);
    }
    let mut max_dist = None;
    if let Some(neighbors) = graph.get(&pos) {
        for &(next_pos, d) in neighbors {
            if visited.insert(next_pos) {
                if let Some(dist) = dfs(graph, visited, &next_pos, &end_pos) {
                    max_dist = Some(max_dist.unwrap_or(0).max(d + dist));
                }
                visited.remove(&next_pos);
            }
        }
    }
    max_dist
}

fn find_max_path_len(map: &Map, with_slopes: bool, start_pos: &Pos, end_pos: &Pos) -> u64 {
    let mut graph: AdjacentsMap = HashMap::new();

    for y in 0..map.height {
        for x in 0..map.width {
            if map.tiles[y][x] == '#' {
                continue;
            }
            let e = graph.entry(Pos { x, y }).or_default();
            for pos in map.neighbors(&Pos { x, y }, with_slopes) {
                e.push((pos, 1));
            }
        }
    }
    let to_join: Vec<Pos> = graph.iter()
        .filter(|(_, neighbors)| neighbors.len() == 2)
        .map(|(&pos, _)| pos)
        .collect();

    for &pos in &to_join {
        let neighbors = graph.remove(&pos).unwrap();

        let (pos1, dist1) = neighbors[0];
        let (pos2, dist2) = neighbors[1];

        if let Some(edges) = graph.get_mut(&pos1) {
            if let Some(edge) = edges.iter_mut().find(|(edge_pos, _)| *edge_pos == pos) {
                *edge = (pos2, dist1 + dist2);
            }
        }
        if let Some(edges) = graph.get_mut(&pos2) {
            if let Some(edge) = edges.iter_mut().find(|(edge_pos, _)| *edge_pos == pos) {
                *edge = (pos1, dist1 + dist2);
            }
        }
    }

    dfs(&graph, &mut HashSet::new(), start_pos, end_pos).unwrap() as u64
}


fn parse_input(input: &str) -> (Map, Pos, Pos) {
    let map = Map::from_str(input);
    let start_pos = Pos { x: 1, y: 0 };
    let end_pos = Pos { x: map.width-2, y: map.height-1 };
    (map, start_pos, end_pos)
}

pub fn part1(input: String) -> u64 {
    let (map, start_pos, end_pos) = parse_input(&input);
    find_max_path_len(&map, true, &start_pos, &end_pos)
}

pub fn part2(input: String) -> u64 {
    let (map, start_pos, end_pos) = parse_input(&input);
    find_max_path_len(&map, false, &start_pos, &end_pos)
}