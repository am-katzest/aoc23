use std::{collections::HashMap, ops::Index, ops::IndexMut};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Target {
    Reject,
    Accept,
    Step(String),
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
struct Step {
    guard: Guard,
    on_true: Target,
    on_false: Target,
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

fn parse_instruction(i: &str) -> (Target, Guard) {
    let (g, t) = i.split(':').collect_tuple().unwrap();
    let guard = parse_guard(g);
    let target = parse_target(t);
    (target, guard)
}

fn parse_target(t: &str) -> Target {
    match t {
        "A" => Target::Accept,
        "R" => Target::Reject,
        x => Target::Step(String::from(x)),
    }
}

fn parse_workflow_unroll(w: &str) -> Vec<(String, Step)> {
    let (name, c, _) = w.split(|x| x == '}' || x == '{').collect_tuple().unwrap();
    let mut is = c.split(',').rev();
    let default = parse_target(is.next().unwrap());
    let basename = String::from(name);
    let mut pos = c.split(',').count() - 1;
    let mut acc = vec![];
    let mut on_false = default;
    // we are moving from the back to the front, splitting workflow into steps
    for (on_true, guard) in is.map(parse_instruction) {
        pos -= 1;
        let name = match pos {
            0 => basename.to_owned(),
            x => format!("{}{}", basename, x),
        };
        acc.push((name.to_owned(), Step { on_true, on_false, guard }));
        on_false = Target::Step(name);
    }
    acc
}

type Steps = HashMap<String, Step>;
type Data = (Steps, Vec<Part>);

fn parse(f: &str) -> Data {
    let content = std::fs::read_to_string(f).unwrap();
    let (w, p) = content.split("\n\n").collect_tuple().unwrap();
    let steps: Steps = w.lines().flat_map(parse_workflow_unroll).collect();
    let parts = p.lines().map(parse_part).collect();
    (steps, parts)
}

fn matches(g: Guard, p: Part) -> bool {
    match g.op {
        Op::Greater => p[g.key] > g.value,
        Op::Lesser => p[g.key] < g.value,
    }
}

fn next(s: Step, p: Part) -> Target {
    if matches(s.guard, p) {
        s.on_true
    } else {
        s.on_false
    }
}

fn accepted(wfs: &Steps, p: Part, current: String) -> bool {
    match next(wfs.get(&current).unwrap().to_owned(), p) {
        Target::Reject => false,
        Target::Accept => true,
        Target::Step(x) => accepted(wfs, p, x),
    }
}

fn sum(p: Part) -> isize {
    p.x + p.m + p.a + p.s
}

fn length(r: Range) -> isize {
    r.max - r.min + 1
}

fn possibilities(p: PartRange) -> isize {
    length(p.x) * length(p.m) * length(p.a) * length(p.s)
}

fn part1((ws, ps): Data, initial: String) -> isize {
    ps.into_iter()
        .filter(|&p| accepted(&ws, p, initial.to_owned()))
        .fold(0, |acc, x| acc + sum(x))
}

// inclusive
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Range {
    min: isize,
    max: isize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl Index<Key> for PartRange {
    type Output = Range;
    fn index(&self, k: Key) -> &Range {
        match k {
            Key::X => &self.x,
            Key::M => &self.m,
            Key::A => &self.a,
            Key::S => &self.s,
        }
    }
}

impl IndexMut<Key> for PartRange {
    fn index_mut(&mut self, k: Key) -> &mut Range {
        match k {
            Key::X => &mut self.x,
            Key::M => &mut self.m,
            Key::A => &mut self.a,
            Key::S => &mut self.s,
        }
    }
}

fn below(p: isize, r: Range) -> Option<Range> {
    if r.min >= p {
        None
    } else {
        Some(Range { max: r.max.min(p - 1), ..r })
    }
}
fn above(p: isize, r: Range) -> Option<Range> {
    if r.max <= p {
        None
    } else {
        Some(Range { min: r.min.max(p + 1), ..r })
    }
}

// (matching, notmatching)
fn split(g: Guard, r: Range) -> (Option<Range>, Option<Range>) {
    match g.op {
        Op::Lesser => (below(g.value, r), above(g.value - 1, r)),
        Op::Greater => (above(g.value, r), below(g.value + 1, r)),
    }
}
fn split_part(g: Guard, r: PartRange) -> (Option<PartRange>, Option<PartRange>) {
    let (a, b) = split(g, r[g.key]);
    let merge = |nr: Range| {
        let mut u = r.clone();
        u[g.key] = nr;
        u
    };
    (a.map(merge), b.map(merge))
}

fn try_follow(acc: &mut Vec<PartRange>, wfs: &Steps, p: PartRange, current: Target) {
    match current {
        Target::Reject => {}
        Target::Accept => acc.push(p),
        Target::Step(x) => follow(acc, wfs, p, x),
    }
}

fn follow(acc: &mut Vec<PartRange>, ss: &Steps, p: PartRange, current: String) {
    let step = ss.get(&current).unwrap();
    let (matching, nonmatching) = split_part(step.guard, p);
    match matching {
        Some(p) => try_follow(acc, ss, p, step.on_true.to_owned()),
        None => {}
    }
    match nonmatching {
        Some(p) => try_follow(acc, ss, p, step.on_false.to_owned()),
        None => {}
    }
}

fn part2(ws: Steps, init: String) -> isize {
    let mut acc: Vec<PartRange> = vec![];
    let full = Range { min: 1, max: 4000 };
    let initial = PartRange {
        x: full,
        m: full,
        a: full,
        s: full,
    };
    follow(&mut acc, &ws, initial, init);
    acc.into_iter().map(possibilities).sum()
}

fn main() {
    println!("part1: {}", part1(parse("inputs/19b"), String::from("in")));
    println!("part2: {}", part2(parse("inputs/19b").0, String::from("in")));
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
        assert_eq!((Target::Accept, g), parse_instruction("m>2090:A"));
        let (at, ag) = parse_instruction("a<2006:qkq");
        let (bt, bg) = parse_instruction("m>2090:A");
        let default = parse_target("rfg");
        let s1 = Step {
            on_true: at,
            on_false: Target::Step(String::from("px1")),
            guard: ag,
        };
        let s2 = Step {
            on_true: bt,
            on_false: default,
            guard: bg,
        };
        assert_eq!(
            vec![(String::from("px1"), s2), (String::from("px"), s1)],
            parse_workflow_unroll("px{a<2006:qkq,m>2090:A,rfg}")
        );
    }
    #[test]
    fn splitting_test() {
        let s = Range { min: 0, max: 10 };
        let mkr = |min, max| Some(Range { min, max });
        assert_eq!(mkr(3, 10), above(2, s));
        assert_eq!(mkr(0, 10), above(-5, s));
        assert_eq!(mkr(0, 10), above(-1, s));
        assert_eq!(mkr(1, 10), above(0, s));
        assert_eq!(mkr(9, 10), above(8, s));
        assert_eq!(mkr(10, 10), above(9, s));
        assert_eq!(None, above(10, s));

        assert_eq!(mkr(0, 1), below(2, s));
        assert_eq!(mkr(0, 10), below(11, s));
        assert_eq!(mkr(0, 9), below(10, s));
        assert_eq!(mkr(0, 0), below(1, s));
        assert_eq!(None, below(0, s));
        let b4 = Guard {
            op: Op::Lesser,
            value: 4,
            key: Key::X,
        };
        assert_eq!((mkr(0, 3), mkr(4, 10)), split(b4, s));
    }
}
