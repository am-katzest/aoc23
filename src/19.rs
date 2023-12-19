use std::ops::Index;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Target {
    Reject,
    Accept,
    Workflow(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Op {
    Greater,
    Lesser,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Key {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Guard {
    op: Op,
    value: isize,
    key: Key,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Instruction {
    target: Target,
    guard: Guard,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Workflow {
    guards: Vec<Instruction>,
    default: Target,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl Index<Key> for Part {
    type Output = isize;
    fn index(&self, k: Key) -> &isize {
        match k {
            Key::X => &self.x,
            Key::M => &self.m,
            Key::A => &self.a,
            Key::S => &self.s,
        }
    }
}

fn main() {}
