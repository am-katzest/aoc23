use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

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

type Counter = (usize, usize);
fn inc_counter(c: &mut Counter, s: Pulse) {
    match s {
        Pulse::High => c.1 += 1,
        Pulse::Low => c.0 += 1,
    }
}

fn button(n: Network, observed: &mut HashMap<(String, String), isize>) -> (Network, Counter) {
    let mut queue: std::collections::VecDeque<(String, String, Pulse)> = std::collections::VecDeque::new();
    let mut counter = (0, 0);
    queue.push_front((String::from("button"), String::from("broadcaster"), Pulse::Low));
    let mut n = n.clone();
    loop {
        match queue.pop_back() {
            None => break,
            Some((source, current, pulse)) => {
                inc_counter(&mut counter, pulse);
                match observed.get_mut(&(source.to_owned(), current.to_owned())) {
                    Some(x) => {
                        if pulse == Pulse::High {
                            *x = (*x) + 1;
                        }
                    }
                    None => {}
                }
                match n.get_mut(&current) {
                    None => {} // nonexistent output
                    Some(mut machine) => {
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
    (n, counter)
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

fn collect_influence(name: String, network: &Network) -> HashSet<String> {
    let mut acc: HashSet<String> = HashSet::new();
    fn step(network: &Network, acc: &mut HashSet<String>, current: String) {
        if acc.insert(current.to_owned()) {
            match network.get(&current) {
                None => {}
                Some(m) => {
                    for target in m.targets.to_owned() {
                        step(network, acc, target);
                    }
                }
            }
        }
    }
    step(network, &mut acc, name.to_owned());
    acc.remove(&name); //TODO is this needed
    acc
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
    let mut dummy: HashMap<(String, String), isize> = HashMap::new();
    let (l, h) = std::iter::successors(Some((n, (0, 0))), move |(n, _)| Some(button(n.clone(), &mut dummy)))
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
    println!("part1: {:?}", part1(parse("inputs/20b")));
    println!("part2: {:?}", part2(parse("inputs/20b")));
}

fn part2(network: Network) -> usize {
    let impacts = calc_total_impact(network.clone());
    let merger = find_merger(&impacts);
    let last_ones = match network.get(&merger).unwrap().module.to_owned() {
        Module::Conj(x) => x.into_iter().map(|(x, _)| x).collect_vec(),
        _ => panic!(),
    };
    let obs: HashMap<(String, String), isize> = last_ones.to_owned().iter().map(|x| ((x.to_owned(), merger.to_owned()), 0)).collect();
    let mut cycles: HashMap<String, usize> = HashMap::new();
    let mut s = network.clone();
    let mut i = 0;
    loop {
        i = i + 1;
        let mut o = obs.clone();
        s = button(s.to_owned(), &mut o).0;
        for ((key, _), val) in o {
            if val > 0 {
                cycles.insert(key, i);
                if cycles.len() == last_ones.len() {
                    return cycles.iter().fold(1, |acc, (_, x)| acc * x);
                }
            }
        }
    }
}

fn find_merger(impacts: &HashMap<String, HashSet<String>>) -> String {
    for (n, s) in impacts {
        if s.len() == 1 {
            return n.to_owned();
        }
    }
    panic!();
}

fn calc_total_impact(network: HashMap<String, Machine>) -> HashMap<String, HashSet<String>> {
    let mut impact: HashMap<String, HashSet<String>> = HashMap::new();
    for (n, _) in network.to_owned() {
        impact.insert(n.to_owned(), collect_influence(n.to_owned(), &network));
    }
    impact
}
