const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

enum Color {
    Red,
    Green,
    Blue,
}

struct Cube {
    color: Color,
    num: u32,
}

type Round = Vec<Cube>;

struct Game {
    num: u32,
    rounds: Vec<Round>,
}

fn parse_color(s: &str) -> Color {
    match s {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!("Unknown color {}", s),
    }
}

fn parse_cube(s: &str) -> Cube {
    let mut iter = s.split_whitespace();
    let num = iter.next().unwrap().parse::<u32>().unwrap();
    let color = parse_color(iter.next().unwrap());
    Cube { color, num }
}

fn parse_round(input: &str) -> Round {
    input.split(',').map(parse_cube).collect()
}

fn parse_game(input: &str) -> Game {
    let (num_part, rounds_part) = input.split_once(": ").unwrap();
    let num = num_part
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let rounds = rounds_part.split(';').map(parse_round).collect();
    Game { num, rounds }
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

pub fn part1(input: String) -> u64 {
    let games = parse_games(&input);

    let mut sum = 0;
    for game in games.iter() {
        let mut game_ok = true;
        for round in game.rounds.iter() {
            for cube in round.iter() {
                match cube.color {
                    Color::Red => {
                        if cube.num > MAX_RED {
                            game_ok = false;
                        }
                    }
                    Color::Green => {
                        if cube.num > MAX_GREEN {
                            game_ok = false;
                        }
                    }
                    Color::Blue => {
                        if cube.num > MAX_BLUE {
                            game_ok = false;
                        }
                    }
                }
            }
        }
        if game_ok {
            sum += game.num;
        }
    }
    println!("TOTAL: {}", sum);
    sum as u64
}

pub fn part2(input: String) -> u64 {
    let games = parse_games(&input);

    let mut sum = 0;
    for game in games.iter() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in game.rounds.iter() {
            for cube in round.iter() {
                match cube.color {
                    Color::Red => {
                        if cube.num > min_red {
                            min_red = cube.num;
                        }
                    }
                    Color::Green => {
                        if cube.num > min_green {
                            min_green = cube.num;
                        }
                    }
                    Color::Blue => {
                        if cube.num > min_blue {
                            min_blue = cube.num;
                        }
                    }
                }
            }
        }
        sum += min_red * min_green * min_blue;
    }
    println!("TOTAL: {}", sum);
    sum as u64
}
