pub fn part1(input: String) -> u64 {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let mut first = 0;
        let mut last = 0;
        let mut first_found = false;
        for c in line.chars() {
            if c.is_digit(10) {
                if !first_found {
                    first = c.to_digit(10).unwrap();
                    first_found = true;
                }
                last = c.to_digit(10).unwrap();
            }
        }
        sum += 10 * first + last;
    }

    println!("TOTAL: {}", sum);
    sum as u64
}

fn get_num(line: &str) -> u32 {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
    let mut numbers = Vec::new();

    for (i, ch) in line.char_indices() {
        if ch.is_digit(10) {
            numbers.push(ch.to_digit(10).unwrap());
        } else {
            for (idx, digit) in digits.iter().enumerate() {
                if line.get(i..i + digit.len()) == Some(digit) {
                    numbers.push((idx as u32) + 1);
                }
            }
        }
    }
    numbers.first().unwrap() * 10 + numbers.last().unwrap()
}

pub fn part2(input: String) -> u64 {
    input.lines().map(get_num).sum::<u32>() as u64
}
