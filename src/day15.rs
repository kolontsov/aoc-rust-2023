use regex::Regex;
use linked_hash_map::LinkedHashMap;

fn hash(str: &str) -> u8 {
    let mut total: u8 = 0;
    for c in str.chars() {
        total = total.wrapping_add(c as u8).wrapping_mul(17);
    }
    total
}

pub fn part1(input: String) -> u64 {
    assert!(hash(&"HASH") == 52);
    input.trim().split(",").map(|s| hash(s) as u64).sum::<u64>()
}

enum CmdOp { Set, Remove }
struct Cmd<'a> {
    prefix: &'a str,
    op: CmdOp,
    arg: Option<u64>
}

fn parse_cmd(input: &str) -> Cmd {
    let re = Regex::new(r"^([a-zA-Z]+)([-=])(\d+)?$").unwrap();
    let cap = re.captures(input).expect("Incorrect cmd format");

    let prefix = cap.get(1).expect("Missing prefix").as_str();
    let op = match cap.get(2).expect("Missing op").as_str() {
        "-" => CmdOp::Remove,
        "=" => CmdOp::Set,
         _ => unreachable!()
    };
    let arg = cap.get(3).and_then(|m| m.as_str().parse::<u64>().ok());
    Cmd { prefix, op, arg}
}

pub fn part2(input: String) -> u64 {
    let mut boxes: [LinkedHashMap<&str, u64>; 256] = std::array::from_fn(|_| LinkedHashMap::new());
   
    for cmd in input.trim().split(",").map(parse_cmd) {
        let hashmap = &mut boxes[hash(&cmd.prefix) as usize];
        match (cmd.op, cmd.arg) {
            (CmdOp::Set, Some(arg)) => { *hashmap.entry(cmd.prefix).or_insert(0) = arg; },
            (CmdOp::Remove, _) => { hashmap.remove(cmd.prefix); },
            (_, _) => unreachable!()
        };
    }
    boxes.iter().enumerate().flat_map(| (box_id, hashmap) | {
        hashmap.iter().enumerate().map(move |(idx, (_, x))| {
            (box_id as u64+1)*(idx as u64+1)*x
        })
    }).sum()
}
