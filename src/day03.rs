fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
} 

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn part1(input: String) -> u64 {
    let field = parse(&input);  
    let max_x = field[0].len();
    let max_y = field.len();
    let mut total = 0;
    for y in 0..max_y {
        let mut num = 0;
        let mut symbols_around = 0;
        for x in 0..max_x {
            let touches_symbol = NEIGHBORS.iter().filter(|&(dx, dy)| {
                let nx = (x as isize) + dx;
                let ny = (y as isize) + dy;
                if nx < 0 || ny < 0 { return false };
                
                let nx = nx as usize;
                let ny = ny as usize;
                if nx == max_x || ny == max_y { return false };
                
                let ch = field[ny][nx];
                !ch.is_digit(10) && ch!='.'
            }).count() > 0;

            let ch = field[y][x];
            if ch.is_digit(10) {
                if touches_symbol { symbols_around += 1 };
                num *= 10;
                num += ch.to_digit(10).unwrap();
            } else {
                if num > 0 && symbols_around > 0 {
                    total += num;
                }
                num = 0;
                symbols_around = 0;
            }
        }
        if num > 0 && symbols_around > 0 {
            total += num;
        }
    }

    total as u64
}

pub fn part2(input: String) -> u64 {
    let field = parse(&input);  
    let max_x = field[0].len();
    let max_y = field.len();
    let mut total = 0;
    let mut all_pos = Vec::new();
    for y in 0..max_y {
        let mut num = 0;
        let mut symbols_around = 0;
        let mut pos = Vec::new();
        for x in 0..max_x {
            let touches_symbol = NEIGHBORS.iter().filter(|&(dx, dy)| {
                let nx = (x as isize) + dx;
                let ny = (y as isize) + dy;
                if nx < 0 || ny < 0 { return false };
                
                let nx = nx as usize;
                let ny = ny as usize;
                if nx == max_x || ny == max_y { return false };
                
                let ch = field[ny][nx];
                !ch.is_digit(10) && ch!='.'
            }).count() > 0;

            let ch = field[y][x];
            if ch.is_digit(10) {
                if touches_symbol { symbols_around += 1 };
                num *= 10;
                num += ch.to_digit(10).unwrap();
                pos.push((x, y));
            } else {
                if num > 0 && symbols_around > 0 {
                    all_pos.push((num, pos.clone()));
                }
                num = 0;
                symbols_around = 0;
                pos = Vec::new();
            }
        }
        if num > 0 && symbols_around > 0 {
            all_pos.push((num, pos.clone()));
        }
    }
    for y in 0..max_y {
        for x in 0..max_x {
            let ch = field[y][x];
            if ch != '*' { continue; }
            
            let mut my_all_pos = all_pos.clone();
            let mut nums = Vec::new();
            for &(dx, dy) in NEIGHBORS.iter() {
                let nx = (x as isize) + dx;
                let ny = (y as isize) + dy;
                if nx < 0 || ny < 0 { continue };
                
                let nx = nx as usize;
                let ny = ny as usize;
                if nx == max_x || ny == max_y { continue };

                for &mut (ref mut num, ref mut pos) in my_all_pos.iter_mut()  {
                    if *num>0 && pos.contains(&(nx, ny)) {
                        nums.push(*num);
                       *num = 0
                    }
                }
            }
            if nums.len() == 2 {
                total += nums[0] * nums[1];
            }
        }
    }

    total as u64
}
