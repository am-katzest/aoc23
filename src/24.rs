use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_vector(l: &str) -> Vector {
    let (x, y, z) = l.split(|x| x == ' ' || x == ',').filter_map(|x| x.parse::<i32>().ok()).collect_tuple().unwrap();
    Vector {x, y, z}
}
fn parse_hail(h: &str) -> Hail {
    let (position, velocity) = h.split('@').map(parse_vector).collect_tuple().unwrap();
    Hail {position, velocity}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Hail {
    position: Vector,
    velocity: Vector,
}

fn parse(f:&str) -> Vec<Hail> {
    std::fs::read_to_string(f).unwrap().lines().map(parse_hail).collect()
}

fn main() {
    println!("{:?}", parse("inputs/24a"));
}
