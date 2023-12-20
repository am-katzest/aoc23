use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Pulse {
    High,
    Low,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum FlipFlopState {
    On,
    Off,
}

type ConjState = Vec<(String, Pulse)>; // hashmap is an overkill and not hashable, needs to be initialized beforehand

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Module {
    Broadcast,
    FlipFlop(FlipFlopState),
    Conj(ConjState),
}

fn flip(s: FlipFlopState) -> FlipFlopState {
    match s {
        FlipFlopState::On => FlipFlopState::Off,
        FlipFlopState::Off => FlipFlopState::On,
    }
}

fn ffpulse(s: FlipFlopState) -> Pulse {
    match s {
        FlipFlopState::On => Pulse::Low,
        FlipFlopState::Off => Pulse::High,
    }
}

fn apply_flip_flop(s: FlipFlopState, p: Pulse) -> (FlipFlopState, Option<Pulse>) {
    match p {
        Pulse::High => (s, None),
        Pulse::Low => (flip(s), Some(ffpulse(s))),
    }
}

fn apply_conj(s: ConjState, source: String, p: Pulse) -> (ConjState, Option<Pulse>) {
    let s: ConjState = s.into_iter().map(|(n, o)| if n == source { (n, p) } else { (n, o) }).collect_vec();
    if s.iter().all(|(_, s)| *s == Pulse::High) {
        (s, Some(Pulse::Low))
    } else {
        (s, Some(Pulse::High))
    }
}

// what module outputs
fn apply(m: Module, source: String, p: Pulse) -> (Module, Option<Pulse>) {
    // too lazy to figure out how to do polymorphism the *not-hurting* way
    match m {
        Module::Broadcast => (Module::Broadcast, Some(p)),
        Module::FlipFlop(s) => {
            let (s, o) = apply_flip_flop(s, p);
            (Module::FlipFlop(s), o)
        }
        Module::Conj(s) => {
            let (s, o) = apply_conj(s, source, p);
            (Module::Conj(s), o)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ModuleType {
    // just for parsing
    Broadcast,
    FlipFlop,
    Conj,
}

fn pre_parse_module(line: &str) -> (ModuleType, String, Vec<String>) {
    // %jf -> cr, dn
    let kind = match line.chars().next().unwrap() {
        '&' => ModuleType::Conj,
        '%' => ModuleType::FlipFlop,
        _ => ModuleType::Broadcast,
    };
    let mut tmp = line
        .split(|x| x == ' ' || x == '-' || x == '>' || x == ',' || x == '&' || x == '%')
        .filter(|x| x.len() > 0)
        .map(|x| String::from(x));
    let first = tmp.next().unwrap();
    (kind, first, tmp.collect_vec())
}

fn crate_module(t: ModuleType, inputs: Vec<String>) -> Module {
    match t {
        ModuleType::Broadcast => Module::Broadcast,
        ModuleType::FlipFlop => Module::FlipFlop(FlipFlopState::Off),
        ModuleType::Conj => {
            let state = inputs.into_iter().map(|x| (x, Pulse::Low)).collect_vec();
            Module::Conj(state)
        }
    }
}

fn create_machine(t: ModuleType, inputs: Vec<String>, outputs: Vec<String>) -> Machine {
    Machine {
        targets: outputs,
        module: crate_module(t, inputs),
    }
}

type Network = HashMap<String, Machine>;

fn button(n: Network) -> (Network, (usize, usize)) {
    let mut queue: std::collections::VecDeque<(String, String, Pulse)> = std::collections::VecDeque::new();
    let mut high = 0;
    let mut low = 0;
    queue.push_front((String::from("button"), String::from("broadcaster"), Pulse::Low));
    let mut n = n.clone();
    loop {
        match queue.pop_back() {
            None => break,
            Some((source, current, pulse)) => {
                match pulse {
                    Pulse::High => high += 1,
                    Pulse::Low => low += 1,
                }
                //println!("{source} --{:?}--> {current}", pulse);
                match n.get_mut(&current) {
                    None => {} // nonexistent output
                    Some(mut machine) => {
                        //println!("{source} --{:?}--> {current} state: {:?}", pulse, machine);
                        let (module, response) = apply(machine.module.to_owned(), source, pulse);
                        machine.module = module; // update
                        match response {
                            Some(pulse) => {
                                for target in machine.targets.clone() {
                                    queue.push_front((current.to_owned(), target, pulse));
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }
    (n, (low, high))
}

fn parse(f: &str) -> Network {
    let content = read_to_string(f).unwrap();
    let modules = content.lines().map(pre_parse_module).collect_vec();
    let inputs = collect_inputs(&modules);
    modules
        .clone()
        .into_iter()
        .map(|(kind, name, outputs)| {
            (
                //key, value
                name.to_owned(),
                create_machine(kind, inputs.get(&name).unwrap_or(&vec![]).to_owned(), outputs),
            )
        })
        .collect()
}

fn collect_inputs(modules: &Vec<(ModuleType, String, Vec<String>)>) -> HashMap<String, Vec<String>> {
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (_, sender, outputs) in modules.clone() {
        for o in outputs {
            match inputs.get_mut(&o) {
                Some(previous) => {
                    (*previous).push(sender.clone());
                }
                None => {
                    inputs.insert(o, vec![sender.clone()]);
                }
            }
        }
    }
    inputs
}

fn part1(n: Network) -> usize {
    let (l, h) = std::iter::successors(Some((n, (0, 0))), move |(n, _)| Some(button(n.clone())))
        .take(1001)
        .map(|(_, c)| c)
        .reduce(|(al, ah), (bl, bh)| (al + bl, ah + bh))
        .unwrap();
    l * h
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Machine {
    module: Module,
    targets: Vec<String>,
}

fn main() {
    println!("part1: {:?}", part1(parse("inputs/20a")));
    println!("part1: {:?}", part1(parse("inputs/20c")));
    println!("part1: {:?}", part1(parse("inputs/20b")));
}
