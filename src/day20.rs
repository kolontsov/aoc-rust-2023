use std::collections::HashMap;

#[derive(Debug)]
enum ModuleFunc {
    FlipFlop { state: bool }, 
    Conjunction { state: HashMap<String, bool> },
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    func: ModuleFunc,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Pulse {
    src: String,
    dest: String,
    value: bool,
}

type Machine = HashMap<String, Module>;

fn parse_modules(input: &str) -> Machine {
    let mut machine = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").expect("Invalid line");
        let outputs: Vec<String> = right.split(", ").map(String::from).collect();
        if left == "broadcaster" {
            machine.insert(left.to_string(), Module{func: ModuleFunc::Broadcaster, outputs});
            continue;
        }
        let (func_code, name) = left.split_at(1);
        let func = match func_code {
            "%" => ModuleFunc::FlipFlop { state: false },
            "&" => ModuleFunc::Conjunction { state: HashMap::new() },
            _ => unreachable!(),
        };
        let outputs = right.split(", ").map(String::from).collect();
        machine.insert(name.to_string(), Module{func, outputs});
    }
    propagate_inputs(&mut machine);
    machine
}

fn propagate_inputs(machine: &mut Machine) {
    let mut updates = Vec::new();
    for (name, module) in machine.iter() {
        for output in &module.outputs {
            updates.push((output.clone(), name.clone()));
        }
    }    
    for (output, name) in updates {
        if let Some(target_module) = machine.get_mut(&output) {
            match &mut target_module.func {
                ModuleFunc::Conjunction { state } => {
                    state.insert(name, false);
                },
                _ => (),
            };
        }
    }
}

fn process_pulse(p: Pulse, machine: &mut Machine) -> Vec<Pulse> {
    let mut pulse_out = Vec::new();
    if let Some(module) = machine.get_mut(&p.dest) {
        let value = match &mut module.func {
            ModuleFunc::FlipFlop { state } => {
                if p.value == true {
                    return Vec::new();
                }
                *state = !*state;
                *state
            }
            ModuleFunc::Conjunction { state } => {
                state.insert(p.src.to_string(), p.value);
                !state.values().all(|&x| x)
    
            },
            ModuleFunc::Broadcaster => p.value,
        };
        for output in &module.outputs {
            pulse_out.push(Pulse { src: p.dest.to_string(), dest: output.to_string(), value });
        }
    }
    pulse_out
}

// Технология, 1991
fn press_the_button(pulses: &mut Vec<Pulse>){
    pulses.push(Pulse{ src: "button".to_string(), dest: "broadcaster".to_string(), value: false });
}

pub fn part1(input: String) -> u64 {
    let mut machine = parse_modules(&input);

    let mut pulse_cnt = (0, 0);
    for _ in 0..1000 {
        let mut pulses = Vec::new();
        press_the_button(&mut pulses);
    
        while pulses.len()>0 {
            let mut pulse_out: Vec<Pulse> = Vec::new();
            for pulse in pulses.drain(..) {
                match pulse.value {
                    true => pulse_cnt.1 += 1,
                    false => pulse_cnt.0 += 1,
                }
                let out = &mut process_pulse(pulse, &mut machine);
                pulse_out.append(out);
            }
            pulses.append(&mut pulse_out);
        }
    }        
    (pulse_cnt.0 * pulse_cnt.1) as u64
}

pub fn part2(input: String) -> u64 {
    let mut machine = parse_modules(&input);

    let rx_src = machine.values().find(|m| m.outputs.contains(&"rx".to_string()));
    if rx_src.is_none() {
        // println!("No RX, skipping");
        return 0
    }
    
    let rx_depends_on = match rx_src.unwrap().func {
        ModuleFunc::Conjunction { ref state } => state.keys(),
        _ => panic!("RX source is not Conjunction"),
    };
    // println!("RX inputs: {:?}", rx_depends_on);

    let mut high_pulse_detector: HashMap<String, u64> = HashMap::new();
    rx_depends_on.for_each(|k| { high_pulse_detector.insert(k.to_string(), 0); });

    let mut i = 0;
    while high_pulse_detector.values().any(|&x| x==0) {
        i += 1;

        let mut pulses = Vec::new();
        press_the_button(&mut pulses);

        while pulses.len()>0 {
            if pulses.is_empty() { break; }
            let mut pulse_out: Vec<Pulse> = Vec::new();        
            for pulse in pulses.drain(..) {
                let out = &mut process_pulse(pulse, &mut machine);
                pulse_out.append(out);
            }            

            // detect if any of modules which RX depends on just generated high pulse
            for pulse in &pulse_out {
                for (k, v) in high_pulse_detector.iter_mut() {
                    if *v!=0 { continue; }
                    if pulse.src==*k && pulse.value==true {
                        *v = i;
                    }
                }
            }
            pulses.append(&mut pulse_out);    
        }
    }

    high_pulse_detector.values().fold(1, |acc, num| lcm(acc, *num))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}