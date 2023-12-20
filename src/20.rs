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
enum ModuleType { // just for parsing
    Broadcast, FlipFlop, Conj
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

fn parse(f: &str) {
    let content = read_to_string(f).unwrap();
    let modules = content.lines().map(pre_parse_module).collect_vec();

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
    println!("{:?}", modules);
    println!("{:?}", inputs);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Machine {
    kind: Module,
    targets: Vec<String>,
}

fn main() {
    parse("inputs/20a")
}
