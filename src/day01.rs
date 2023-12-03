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
        sum += 10*first+last;
    }

    println!("TOTAL: {}", sum);
    sum as u64
}

fn get_first_last(line: &str) -> (Option<u32>, Option<u32>) {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut first = None;
    let mut last = None;

    for (i, ch) in line.char_indices() {
        if ch.is_digit(10) {
            let value = ch.to_digit(10).unwrap();
            first.get_or_insert(value);
            last = Some(value);
        } else {
            for (idx, digit) in digits.iter().enumerate() {
                if line.get(i..i+digit.len()) == Some(digit) {
                    let value = (idx as u32)+1;
                    first.get_or_insert(value);
                    last = Some(value);
                }
            }
        }
    }
    (first, last)
}

pub fn part2(input: String) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        if let (Some(first), Some(last)) = get_first_last(line) {        
            // println!("{} {}", first_num, last_num);
            sum += 10*first+last;
        }
    }

    println!("TOTAL: {}", sum);
    sum as u64
}