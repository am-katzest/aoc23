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

fn button(n: Network) -> (Network, Counter, Counter) {
    let mut queue: std::collections::VecDeque<(String, String, Pulse)> = std::collections::VecDeque::new();
    let mut counter = (0, 0);
    let mut rxcounter = (0, 0);
    queue.push_front((String::from("button"), String::from("broadcaster"), Pulse::Low));
    let mut n = n.clone();
    loop {
        match queue.pop_back() {
            None => break,
            Some((source, current, pulse)) => {
                inc_counter(&mut counter, pulse);
                //println!("{source} --{:?}--> {current}", pulse);
                match n.get_mut(&current) {
                    None => {
                        if current == "rx" {
                            inc_counter(&mut rxcounter, pulse);
                        }
                    } // nonexistent output
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
    (n, counter, rxcounter)
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
    let (l, h) = std::iter::successors(Some((n, (0, 0), (0, 0))), move |(n, _, _)| Some(button(n.clone())))
        .take(1001)
        .map(|(_, c, _)| c)
        .reduce(|(al, ah), (bl, bh)| (al + bl, ah + bh))
        .unwrap();
    l * h
}

fn part2(n: Network) -> usize {
    let mut i = 0;
    let mut s = n.clone();
    loop {
        i += 1;
        let r = button(s.clone());
        s = r.0;
        if i % 1000 == 0 {
            println!("{i} {:?}", r.2);
        }
        if r.2 .0 != 0 {
            println!("{i} {:?}", r.2);
            return i;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Machine {
    module: Module,
    targets: Vec<String>,
}

fn main() {
    part3(parse("inputs/20b"));
    //println!("part1: {:?}", part1(parse("inputs/20b")));
    //println!("part2: {:?}", part2(parse("inputs/20b")));
}

fn part3(network: Network) {
    let impacts = calc_total_impact(network.clone());
    let merger = find_merger(&impacts);
    let last_ones = match network.get(&merger).unwrap().module.to_owned() {
        Module::Conj(x) => x.into_iter().map(|(x, _)| x).collect_vec(),
        _ => panic!(),
    };
    println!("{:?}", last_ones);
    let subgraphs = find_subgraphs(last_ones, impacts);
    subgraphs;
    let mut s = network.clone();
    let mut i = 0;
    loop {
        i = i +1;
        s = button(s.to_owned()).0;
        let important_bit = match s.get(&merger).unwrap().clone().module
            {
                Module::Conj(x) => x,
                _ => panic!(),
            };
        println!("{:?}",important_bit );
        break
    }
}

fn find_subgraphs(last_ones: Vec<String>, impacts: HashMap<String, HashSet<String>>) -> HashMap<String, Vec<String>> {
    let subgraphs: HashMap<String, Vec<String>> = last_ones
        .into_iter()
        .map(|i| {
            let subgraph_content = impacts
                .iter()
                .filter(|(_, y)| y.contains(&i.to_owned()))
                .map(|(x, _)| x.to_owned())
                .collect_vec();
            (i, subgraph_content)
        })
        .collect();
    subgraphs
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
