use std::{collections::HashMap, ops::Index};

use itertools::Itertools;

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
    instructions: Vec<Instruction>,
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

fn parse_part(p: &str) -> Part {
    // gods this is so fragile
    let (x, m, a, s) = p
        .split(|x: char| !x.is_ascii_digit())
        .filter_map(|x| x.parse::<isize>().ok())
        .collect_tuple()
        .unwrap();
    Part { x, m, a, s }
}

fn parse_guard(g: &str) -> Guard {
    let (p, op) = if g.contains('>') { ('>', Op::Greater) } else { ('<', Op::Lesser) };
    let (k, v) = g.split(p).collect_tuple().unwrap();
    let value = v.parse().unwrap();
    let key = match k {
        "x" => Key::X,
        "m" => Key::M,
        "a" => Key::A,
        "s" => Key::S,
        _ => panic!("wrong character {k}"),
    };
    Guard { op, value, key }
}

fn parse_instruction(i: &str) -> Instruction {
    let (g, t) = i.split(':').collect_tuple().unwrap();
    let guard = parse_guard(g);
    let target = parse_target(t);
    Instruction { target, guard }
}

fn parse_target(t: &str) -> Target {
    match t {
        "A" => Target::Accept,
        "R" => Target::Reject,
        x => Target::Workflow(String::from(x)),
    }
}

fn parse_workflow(w: &str) -> (String, Workflow) {
    let (name, c, _) = w.split(|x| x == '}' || x == '{').collect_tuple().unwrap();
    let mut is = c.split(',').rev();
    let default = parse_target(is.next().unwrap());
    let instructions = is.rev().map(parse_instruction).collect_vec();
    (String::from(name), Workflow { instructions, default })
}
type Workflows = HashMap<String, Workflow>;
type Data = (Workflows, Vec<Part>);

fn parse(f: &str) -> Data {
    let content = std::fs::read_to_string(f).unwrap();
    let (w, p) = content.split("\n\n").collect_tuple().unwrap();
    let workflows: HashMap<String, Workflow> = w.lines().map(parse_workflow).collect();
    let parts = p.lines().map(parse_part).collect();
    (workflows, parts)
}

fn matches(g: Guard, p: Part) -> bool {
    match g.op {
        Op::Greater => p[g.key] > g.value,
        Op::Lesser => p[g.key] < g.value,
    }
}

fn next(wf: Workflow, p: Part) -> Target {
    for i in wf.instructions {
        if matches(i.guard, p) {
            return i.target;
        }
    }
    wf.default
}

fn accepted(wfs: &Workflows, p: Part, current: String) -> bool {
    match next(wfs.get(&current).unwrap().to_owned(), p) {
        Target::Reject => false,
        Target::Accept => true,
        Target::Workflow(x) => accepted(wfs, p, x),
    }
}

fn sum(p: Part) -> isize {
    p.x + p.m + p.a + p.s
}

fn part1((ws, ps): Data, initial: String) -> isize {
    ps.into_iter()
        .filter(|&p| accepted(&ws, p, initial.to_owned()))
        .fold(0, |acc, x| acc + sum(x))
}

fn main() {
    println!("part1: {}", part1(parse("inputs/19b"), String::from("in")));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parsing_test() {
        assert_eq!(
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            },
            parse_part("{x=787,m=2655,a=1222,s=2876}")
        );
        let g = Guard {
            op: Op::Greater,
            value: 2090,
            key: Key::M,
        };
        assert_eq!(g, parse_guard("m>2090"));
        assert_eq!(
            Instruction {
                guard: g,
                target: Target::Accept
            },
            parse_instruction("m>2090:A")
        );
        let a = parse_instruction("a<2006:qkq");
        let b = parse_instruction("m>2090:A");
        let default = parse_target("rfg");
        let instructions = vec![a, b];
        assert_eq!(
            (String::from("px"), Workflow { instructions, default }),
            parse_workflow("px{a<2006:qkq,m>2090:A,rfg}")
        );
    }
}
