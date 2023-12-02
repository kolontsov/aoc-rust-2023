const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn part1(input: String) {
    let mut sum = 0;

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        iter.next();
        let game_num = iter.next().unwrap().trim_end_matches(':').parse::<u32>().unwrap();
        println!("Game {}", game_num);
        
        let mut game_str = String::new();
        for word in iter {
            game_str.push_str(word);
            game_str.push(' ');
        }
        let mut game_ok = true;
        for (idx, round) in game_str.split(';').enumerate() {
            println!("  Round {}", idx);
            for cube in round.split(',') {
                let mut iter = cube.split_whitespace();
                let num = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap();
                match color {
                    "red" => {
                        if num > MAX_RED { game_ok = false; }
                    },
                    "green" => {
                        if num > MAX_GREEN { game_ok = false; }
                    },
                    "blue" => {
                        if num > MAX_BLUE { game_ok = false; }
                    },
                    _ => {
                        println!("Unknown color: {}", color);
                        std::process::exit(1);
                    }
                }
//                println!("  - {} {}", num, color);
            }
        }
        if game_ok {
            sum += game_num;
        }
    }
    println!("TOTAL: {}", sum);
}

pub fn part2(input: String) {
    let mut sum = 0;

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        iter.next();
        let game_num = iter.next().unwrap().trim_end_matches(':').parse::<u32>().unwrap();
        println!("Game {}", game_num);
        
        let mut game_str = String::new();
        for word in iter {
            game_str.push_str(word);
            game_str.push(' ');
        }
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for (idx, round) in game_str.split(';').enumerate() {
            println!("  Round {}", idx);
            for cube in round.split(',') {
                let mut iter = cube.split_whitespace();
                let num = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap();
                match color {
                    "red" => { if num > min_red { min_red = num; } },
                    "green" => { if num > min_green { min_green = num; } },
                    "blue" => { if num > min_blue { min_blue = num; } },
                    _ => {
                        println!("Unknown color: {}", color);
                        std::process::exit(1);
                    }
                }
                //println!("  - {} {}", num, color);
            }
        }
        let power = min_red*min_green*min_blue;
        sum += power;
    }
    println!("TOTAL: {}", sum);
}