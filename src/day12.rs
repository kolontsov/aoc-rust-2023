use std::collections::HashMap;

fn is_valid_group(chars: &[char], length: usize) -> bool {
    if length > chars.len() || chars[..length].contains(&'.') {
        false
    } else if chars.len() > length {
        matches!(chars[length], '.' | '?')
    } else {
        true
    }
}

fn count_combo(chars: &[char], groups: &&[usize], position: usize, group_id: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    let key = (position, group_id);

        if let Some(&count) = cache.get(&key) {
        return count;
    }

    if group_id == groups.len() {
        if position>=chars.len() || chars[position..].iter().all(|&c| c != '#') { 
            cache.insert(key, 1);
            return 1
        } else { return 0 };        
    }

    if position >= chars.len() {
        return 0;
    }

    if chars[position] == '#' {
        if is_valid_group(&chars[position..], groups[group_id]) {
            let res = count_combo(&chars, groups, position + (groups[group_id] as usize) + 1, group_id + 1, cache);
            cache.insert(key, res);
            return res;
        }
        else {
            return 0;
        }
    }

    if is_valid_group(&chars[position..], groups[group_id]) {
        let res = count_combo(&chars, groups, position + (groups[group_id] as usize) + 1, group_id + 1, cache)
            + count_combo(&chars, groups, position + 1, group_id, cache);
        cache.insert(key, res);
        return res;
    }

    let res = count_combo(&chars, groups, position + 1, group_id, cache);
    cache.insert(key, res);
    res

}

fn count_combinations(chars: &[char], groups: &[usize]) -> u64 {
    let mut cache = HashMap::new();
    count_combo(&chars, &groups, 0, 0, &mut cache)
}    

fn parse_input(input: &str) -> (Vec<char>, Vec<usize>) {
    let (chars_part, groups_part) = input.split_once(" ").unwrap();
    let chars = chars_part.chars().collect();
    let groups = groups_part.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    (chars, groups)
}

pub fn part1(input: String) -> u64 {
    let mut total = 0;
    for (_, line) in input.lines().enumerate() {
        //if i!=5  { continue; }
        let (chars, groups) = parse_input(&line);
       // println!("{}: {:?} {:?}", line, chars, groups);
        let n = count_combinations(&chars, &groups);
       // println!("Combos: {}\n", n);
        total += n;
    }
    total
}

pub fn part2(input: String) -> u64 {
    let mut total = 0;
    for (_, line) in input.lines().enumerate() {
        //if i!=5  { continue; }

        let (chars, groups) = parse_input(&line);

        let mut chars2: Vec<char> = Vec::new();
        for i in 0..5 {
            if i > 0 { chars2.push('?'); }
            chars2.extend(&chars);
        }

        let mut groups2: Vec<usize> = Vec::new();
        for _ in 0..5 {
            groups2.extend(&groups);
        }

        // println!("{}: {:?} {:?}", line, chars, groups);
        let n = count_combinations(&chars2, &groups2);
       // println!("Combos: {}\n", n);
        total += n;
    }
    total
}
