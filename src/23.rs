use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::iter;
use std::ops::Index;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

fn clockwise(x: Dir) -> Dir {
    match x {
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Up => Dir::Right,
    }
}

fn opposite(x: Dir) -> Dir {
    clockwise(clockwise(x))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Addr = u32; // could be u16 ( ***i think*** )

type Coord = (Addr, Addr);

fn step(d: Dir, (x, y): Coord, m: &Map) -> Option<Coord> {
    let (mx, my) = m.size;
    match d {
        Dir::Left if x > 0 => Some((x - 1, y)),
        Dir::Right if x < (mx - 1) => Some((x + 1, y)),
        Dir::Down if y < (my - 1) => Some((x, y + 1)),
        Dir::Up if y > 0 => Some((x, y - 1)),
        _ => None,
    }
}

fn advance(s: Scanner, m: &Map) -> Option<Scanner> {
    let coord = step(s.dir, s.coord, m)?;
    Some(Scanner { coord, ..s })
}

//type Map = Vec<Vec<Tile>>;
#[derive(Clone, Debug)]
struct Map {
    start: Coord,
    end: Coord,
    size: Coord,
    tiles: Vec<Vec<Tile>>,
}

impl Index<Coord> for Map {
    type Output = Tile;
    fn index(&self, (x, y): Coord) -> &Tile {
        &self.tiles[y as usize][x as usize]
    }
}

fn parse_tile(t: char) -> Tile {
    match t {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '>' => Tile::Slope(Dir::Right),
        '<' => Tile::Slope(Dir::Left),
        '^' => Tile::Slope(Dir::Up),
        'v' => Tile::Slope(Dir::Down),
        _ => panic!("the fuck is {t}"),
    }
}

fn parse(f: &str) -> Map {
    let tiles = read_to_string(f)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(parse_tile).collect_vec())
        .collect_vec();
    let size = (tiles[0].len() as Addr, tiles.len() as Addr);
    let start = (1, 0);
    let end = ((size.0 - 2) as Addr, (size.1 - 1) as Addr);
    Map { tiles, start, end, size }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    start: Coord,
    end: Coord,
    enddir: Dir,
    dir: Dir,
    length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Scanner {
    dir: Dir,
    coord: Coord,
}

fn all_around(map: &Map) -> impl Iterator<Item = Scanner> + '_ {
    let down = (0..map.size.0).map(|x| Scanner {
        dir: Dir::Down,
        coord: (x, 0),
    });
    let up = (0..map.size.0).map(|x| Scanner {
        dir: Dir::Up,
        coord: (x, map.size.1 - 1),
    });
    let right = (0..map.size.1).map(|y| Scanner {
        dir: Dir::Right,
        coord: (0, y),
    });
    let left = (0..map.size.1).map(|y| Scanner {
        dir: Dir::Left,
        coord: (map.size.0 - 1, y),
    });
    left.chain(right).chain(up).chain(down)
}

fn traversible_tile(t: Tile, d: Dir) -> bool {
    match t {
        Tile::Path => true,
        Tile::Slope(x) if x == d => true,
        _ => false,
    }
}

fn traversible(m: &Map, s: Scanner) -> bool {
    traversible_tile(m[s.coord], s.dir)
}

fn insert_one(hm: &mut HashMap<Coord, HashSet<Node>>, k: Coord, v: Node) {
    match hm.get_mut(&k) {
        Some(p) => {
            p.insert(v);
        }
        None => {
            let mut n = HashSet::new();
            n.insert(v);
            hm.insert(k, n);
        }
    }
}

fn insert(n: &mut Nodes, v: Node) {
    insert_one(&mut n.starts, v.start, v);
    insert_one(&mut n.ends, v.end, v);
}

fn remove(n: &mut Nodes, v: Node) {
    remove_one(&mut n.starts, v.start, v);
    remove_one(&mut n.ends, v.end, v);
}

fn remove_one(hm: &mut HashMap<Coord, HashSet<Node>>, k: Coord, v: Node) {
    let mut end = hm.get_mut(&k).unwrap();
    end.remove(&v);
    if end.len() == 0 {
        hm.remove(&k);
    }
}

fn find_pairs(m: &Map, initial: Scanner, acc: &mut Nodes) {
    let mut current = advance(initial, m).unwrap();
    let mut last = initial;
    loop {
        if traversible(m, current) && traversible(m, last) {
            let start = last.coord;
            let end = current.coord;
            let node = Node {
                start,
                end,
                dir: initial.dir,
                enddir: initial.dir,
                length: 1,
            };
            insert(acc, node);
        }
        match advance(current, m) {
            Some(next) => {
                last = current;
                current = next;
            }
            None => {
                break;
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Nodes {
    starts: HashMap<Coord, HashSet<Node>>,
    ends: HashMap<Coord, HashSet<Node>>,
}

fn merge2(n: &mut Nodes, head: Node, tail: Node) {
    let merged = Node {
        start: head.start,
        dir: head.dir,
        enddir: tail.enddir,
        end: tail.end,
        length: head.length + tail.length,
    };
    //println!("merging {:?} and {:?} into {:?}", head, tail, merged);
    insert(n, merged);
    remove(n, head);
    remove(n, tail);
}

fn merge(i: Nodes) -> Nodes {
    let mut nodes = i.clone();
    loop {
        let mut to_merge: Option<(Node, Node)> = None; //fighting with borrow checker
        'outer: for (_, ns) in nodes.starts.iter() {
            for this in ns {
                match nodes.starts.get(&this.end) {
                    Some(x) => {
                        let others = x.iter().filter(|x| x.dir != opposite(this.enddir)).collect_vec();
                        if others.len() == 1 {
                            //exactly one in one of the three acceptable directions
                            to_merge = Some((*this, *others[0]));
                            break 'outer;
                        }
                    }
                    _ => {}
                };
            }
        }
        match to_merge {
            None => {
                break;
            }
            Some((this, other)) => {
                merge2(&mut nodes, this, other);
            }
        }
    }
    nodes
}

fn get_nodes(m: Map) -> Nodes {
    let mut acc = Nodes {
        starts: HashMap::new(),
        ends: HashMap::new(),
    };
    for i in all_around(&m) {
        find_pairs(&m, i, &mut acc);
    }
    acc
}

fn rec_part(n: &Nodes, target: Coord, forbidden: Vec<Coord>, current: Node, len: usize) -> usize {
    //TODO end condition
    if current.end == target {
        return len;
    }
    //println!("meow {len} {:?} {:?}", target, current);
    let mut ml = 0;

    match n.starts.get(&current.end) {
        None => 0,
        Some(children) => {
            for child in children {
                if !forbidden.contains(&child.end) {
                    let mut forbidden_child = forbidden.clone();
                    forbidden_child.push(child.end);
                    ml = ml.max(rec_part(n, target, forbidden_child, *child, len + child.length));
                }
            }
            ml
        }
    }
}

fn part1(m: Map, n: Nodes) -> usize {
    let first = *n.starts.get(&m.start).unwrap().iter().next().unwrap();
    let target = m.end;
    rec_part(&n, target, vec![first.start], first, first.length)
}

fn main() {
    let m = parse("inputs/23a");
    let n = merge(get_nodes(m.clone()));
    println!("part 1: {:?}", part1(m.clone(), n.clone()));

    let m1 = parse("inputs/23a");
    let n1 = merge(get_nodes(m1.clone()));
    println!("part 1: {:?}", part1(m1.clone(), n1.clone()));
    println!(
        "{:?}",
        merge(get_nodes(m1.clone()))
            .starts
            .iter()
            .map(|(k, v)| (k, v.iter().sorted().collect_vec()))
            .sorted()
            .collect_vec()
    );
    println!(
        "{:?}",
        merge(get_nodes(m1.clone()))
            .starts
            .iter()
            .map(|(k, v)| (k, v.iter().sorted().collect_vec()))
            .sorted()
            .collect_vec()
    );
    println!(
        "{:?}",
        merge(get_nodes(m1.clone()))
            .starts
            .iter()
            .map(|(k, v)| (k, v.iter().sorted().collect_vec()))
            .sorted()
            .collect_vec()
    );
    println!("{:?}", n1);
}
