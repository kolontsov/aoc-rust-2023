#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>
}

#[derive(Debug, Clone)]
struct Map {
    source: String,
    dest: String,
    ranges: Vec<Range>
}

#[derive(Debug, Clone)]
struct Range {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

fn parse_almanac(input: &str) -> Almanac {
    let mut lines_iter = input.lines();
    
    // "seeds: 79 14 55 13"
    let seeds = lines_iter.next().unwrap()
        .split_once(": ").unwrap().1
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(lines_iter.next().unwrap().is_empty());

    let mut maps: Vec<Map> = vec![];
    while let Some(line) = lines_iter.next() {
        // "soil-to-fertilizer map:"
        let header: Vec<&str> = line.split_once(" ").unwrap().0
            .split("-")
            .collect();
        
        let mut ranges: Vec<Range> = vec![];
        while let Some(line) = lines_iter.next() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<u64> = line.split(" ")
                .map(|s| s.parse().unwrap())
                .collect();
            ranges.push(Range {
                dest_start: parts[0],
                source_start: parts[1],
                length: parts[2]
            });
        }
        maps.push(Map {
            source: header[0].to_string(),
            dest: header[2].to_string(),
            ranges
        });
    }

    Almanac { seeds, maps }
}

pub fn part1(input: String) -> u64 {
    let almanac = parse_almanac(&input);
    let mut cur = "seed";
    let mut values = almanac.seeds.clone();
    loop {
        let map = almanac.maps.iter().find(|&map| map.source == cur).unwrap();
        println!("{} -> {}", map.source, map.dest);
        for i in 0..values.len() {
            let val = values[i];
            for range in &map.ranges {
                if val >= range.source_start && val < range.source_start + range.length {
                    let new_val = val - range.source_start + range.dest_start;
                    println!("  {} -> {}", val, new_val);
                    values[i] = new_val;
                    break;
                }
            }
        }
        cur = map.dest.as_str();
        if cur == "location" {
            break;
        }
    }
    values.iter().min().unwrap().clone() as u64
}

pub fn part2(input: String) -> u64 {
    let almanac = parse_almanac(&input);
    let chunks = almanac.seeds.chunks(2).map(|c| [c[0], c[1]]);
    let mut min = u64::MAX;
    for [x, y] in chunks {
        println!("{} {}, cur min {}", x, y, min);
        for i in x..x+y {
            let mut val = i;
            for map in &almanac.maps {
                for range in &map.ranges {
                    if val>=range.source_start && val<range.source_start+range.length {
                        val = val-range.source_start+range.dest_start;
                        break;
                    }
                }
            }
            if val < min {
                min = val;
            }
        }
    }
    min
}


