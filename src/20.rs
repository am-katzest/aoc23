use std::{collections::HashMap, fs::read_to_string};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Pulse {
    High,
    Low,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum FlipFlopState {
    On, Off
}

type ConjState = Vec<(String, Pulse)>; // hashmap is an overkill and not hashable

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Module {
    Broadcast,
    FlipFlop(FlipFlopState),
    Conj(ConjState)
}

fn flip(s:FlipFlopState) -> FlipFlopState {
    match s {
        FlipFlopState::On => FlipFlopState::Off,
        FlipFlopState::Off => FlipFlopState::On,
    }
}

fn ffpulse(s:FlipFlopState) -> Pulse {
    match s {
        FlipFlopState::On => Pulse::Low,
        FlipFlopState::Off => Pulse::High,
    }
}

fn apply_flip_flop(s:FlipFlopState, p: Pulse)  -> (FlipFlopState, Option<Pulse>){
    match p {
        Pulse::High => (s, None),
        Pulse::Low => (flip(s), Some(ffpulse(s))),
    }
}

fn apply_conj(s:ConjState, p: Pulse) -> (ConjState, Option<Pulse>) {
    todo!()
}

// what module outputs
fn apply(m: Module, p:Pulse) -> (Module, Option<Pulse>){
    // too lazy to figure out how to do polymorphism the *not-hurting* way
    match m {
        Module::Broadcast => (Module::Broadcast, Some(p)),
        Module::FlipFlop(s) => {
            let (s, o) = apply_flip_flop(s, p);
            (Module::FlipFlop(s), o)},
        Module::Conj(s) => {
            let (s, o) = apply_conj(s, p);
            (Module::Conj(s), o)},
    }
}



#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Machine {
    kind: Module,
    targets: Vec<String>,
}

fn main(){

}
