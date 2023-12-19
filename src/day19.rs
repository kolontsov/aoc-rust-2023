use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum Cond { LT, GT }

#[derive(Debug)]
struct Rule {
    cond: Cond,
    cond_var: String,
    cond_value: u64,
    next: String,
}

struct Ruleset {
    rules: Vec<Rule>,
    default_next: String,
}
type Vars = HashMap<String, u64>;
type Ranges = HashMap<String, (u64, u64)>;
type Program = HashMap<String, Ruleset>;

fn parse_input(input: &str) -> (Program, Vec<Vars>){
    let mut program: Program = HashMap::new();
    let mut vars_vec: Vec<Vars> = vec![];

    let re_workflow = Regex::new(r"(\w+)\{(\S+)\}").unwrap();
    let re_rule = Regex::new(r"(\w+)([><])(\d+):(\w+)").unwrap();
    let re_var = Regex::new(r"(\w+)=(\d+)").unwrap();

    let mut input_iter = input.lines().peekable();
    while let Some(_) = input_iter.peek() {
        let line = input_iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let cap = re_workflow.captures_iter(line).next().expect("Invalid workflow");
        let state_name = cap[1].to_string();
        let mut rules = Vec::new();
        let mut default_next = "".to_string();
        for s in cap[2].split(",") {
            if let Some(cap) = re_rule.captures_iter(s).next() {
                let cond_var = cap[1].to_string();
                let cond_value = cap[3].parse::<u64>().unwrap();
                let cond = match &cap[2] {
                    "<" => Cond::LT,
                    ">" => Cond::GT,
                    _ => unreachable!(),
                };
                rules.push(Rule{cond, cond_var, cond_value, next: cap[4].to_string()});
            } else {
                default_next = s.to_string();
                break;
            }
        }
        program.insert(state_name, Ruleset{rules, default_next});
    }

    for line in input_iter {
        let s = &line[1..line.len() - 1];
        let mut vars = HashMap::new();
        for s in s.split(",") {
            let cap = re_var.captures_iter(s).next().expect("Invalid var");
            let var_name = cap[1].to_string();
            let value = cap[2].parse::<u64>().unwrap();
            vars.insert(var_name, value);
        }
        vars_vec.push(vars);
    }
    (program, vars_vec)
}

fn is_accepted(vars: &Vars, program: &Program) -> bool {
    let mut state = "in";
    while state!="R" && state!="A" {
        let ruleset = program.get(state).expect("Missing state");
        state = ruleset.default_next.as_str();
        for rule in &ruleset.rules {
            let var_value = *vars.get(&rule.cond_var).unwrap();
            let success = match &rule.cond {
                Cond::LT => var_value < rule.cond_value,
                Cond::GT => var_value > rule.cond_value,
            };
            if success {
                state = rule.next.as_str();
                break;
            }
        }
    }
    state == "A"
}

pub fn part1(input: String) -> u64 {
    let (program, vars_vec) = parse_input(&input);
    let mut total = 0;
    for vars in vars_vec {
        if is_accepted(&vars, &program) {
           total += vars.values().sum::<u64>();
        }
    }
    total as u64
}

fn split_ranges(rule: &Rule, ranges: &Ranges) -> (Ranges, Ranges) {
    let &cur_range = ranges.get(&rule.cond_var).expect("Missing range");
    let (new_range, updated_range) = match rule.cond {
        Cond::LT => ((cur_range.0, rule.cond_value - 1), (rule.cond_value, cur_range.1)),
        Cond::GT => ((rule.cond_value + 1, cur_range.1), (cur_range.0, rule.cond_value)),
    };

    let mut new_ranges = ranges.clone();
    new_ranges.insert(rule.cond_var.clone(), new_range);

    let mut updated_ranges = ranges.clone();
    updated_ranges.insert(rule.cond_var.clone(), updated_range);

    (new_ranges, updated_ranges)
}

fn count_in_ranges(state: String, ranges: &Ranges, program: &Program) -> u64 {
    if state == "R" { return 0; }
    if state == "A" { return ranges.values().fold(1, |acc, r| acc*(r.1-r.0+1)); }
    let ruleset = program.get(&state).expect("Ruleset not found");
    let mut ranges = ranges.clone();

    let mut total = 0;
    for rule in &ruleset.rules {
        let (new_ranges, updated_ranges) = split_ranges(&rule, &ranges);
        total += count_in_ranges(rule.next.to_string(), &new_ranges, program);
        ranges = updated_ranges;
    }
    total += count_in_ranges(ruleset.default_next.clone(), &ranges, program);

    total
}

pub fn part2(input: String) -> u64 {
    let (program, _) = parse_input(&input);
    let mut ranges = HashMap::new();
    for range_name in ["x", "m", "a", "s"] {
        ranges.insert(range_name.to_string(), (1, 4000));
    }
    count_in_ranges("in".to_string(), &ranges, &program)
}