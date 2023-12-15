use regex::Regex;

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
    let cap = re.captures_iter(input).next().unwrap();

    let prefix = cap.get(1).unwrap().as_str();
    let op = match cap.get(2).unwrap().as_str() {
        "-" => CmdOp::Remove,
        "=" => CmdOp::Set,
         _ => unreachable!()
    };
    let arg = cap.get(3).map_or(None, |m| m.as_str().parse::<u64>().ok());
    Cmd { prefix, op, arg}
}

pub fn part2(input: String) -> u64 {
    let mut boxes: [Vec<(&str, u64)>; 256] = std::array::from_fn(|_| Vec::new());

    for cmd in input.trim().split(",").map(parse_cmd) {
        let vec = &mut boxes[hash(&cmd.prefix) as usize];
        match cmd.op {
            CmdOp::Set => {
                match vec.iter().position(|(p, _)| *p == cmd.prefix) {
                    Some(pos) => vec[pos].1 = cmd.arg.unwrap(),
                    None => vec.push((cmd.prefix, cmd.arg.unwrap())),
                }
            },
            CmdOp::Remove => {
                if let Some(pos) = vec.iter().position(|(p, _)| *p == cmd.prefix) {
                    vec.remove(pos);
                }
            }
        }
    }
    let mut total = 0;
    for i in 0..256 {
        total += boxes[i].iter().enumerate()
            .map(|(idx, (_, x))| (i as u64+1)*(idx as u64+1)*x).sum::<u64>();
    }
    total
}
