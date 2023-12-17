use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};
use std::fs::read_to_string;
use std::ops::Index;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Coord = (usize, usize);

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

fn counterclockwise(x: Dir) -> Dir {
    clockwise(opposite(x))
}

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

type Tile = usize; // TODO check impact of size on performance
#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    size: Coord,
}

impl Index<Coord> for Map {
    type Output = Tile;
    fn index(&self, (x, y): Coord) -> &Tile {
        &self.tiles[y][x]
    }
}

fn parse_tile(t: char) -> Tile {
    t.to_digit(10).unwrap() as Tile
}

fn parse(f: &str) -> Map {
    let tiles = read_to_string(f)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(parse_tile).collect_vec())
        .collect_vec();
    let size = (tiles[0].len(), tiles.len());
    Map { tiles, size }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
// we can't hash for this one ;-;
struct Nav {
    total_cost: usize, // Ord will check this first, rest doesn't matter
    temp_drop: usize,
    dir: Dir,
    coord: Coord,
    tiles_straight: usize,
}
static MAX: usize = 3;
fn create(m: &Map, parent: Nav, dir: Dir) -> Option<Nav> {
    let tiles_straight = if parent.dir == dir {
        if parent.tiles_straight >= MAX {
            return None;
        }
        parent.tiles_straight + 1
    } else {
        1
    };
    match step(dir, parent.coord, m) {
        None => None,
        Some(coord) => {
            let temp_drop = parent.temp_drop + m[coord];
            let dist = to_end(coord, m);
            let total_cost = dist + temp_drop;
            Some(Nav {
                coord,
                dir,
                tiles_straight,
                temp_drop,
                total_cost,
            })
        }
    }
}

fn to_end(u: Coord, m: &Map) -> usize {
    // manhattan, guaranteed to be no less then actual total temp drop
    let (tx, ty) = m.size;
    let (x, y) = u;
    (tx - x) + (ty - y)
}

fn finished(n: Nav, m: &Map) -> bool {
    let (x, y) = n.coord;
    m.size == (x + 1, y + 1)
}

fn part1(f: &str) -> usize {
    let map = parse(f);
    let initial = Nav {
        dir: Dir::Right, // should be able to pivot downwards
        coord: (0, 0),
        temp_drop: 0,
        total_cost: to_end((0, 0), &map),
        tiles_straight: 1,
    };
    let mut ctr = 0;
    let mut queue: BTreeSet<Nav> = BTreeSet::new();
    let mut visited: HashMap<(Coord, Dir, usize), usize> = HashMap::new();
    queue.insert(initial);
    loop {
        let current = queue.pop_first().unwrap(); // we add them faster than we take them
        ctr += 1;
        if finished(current, &map) {
            println!("iterations: {:?}", ctr);
            return current.temp_drop;
        }
        let d = current.dir;
        for dir in [d, clockwise(d), counterclockwise(d)] {
            match create(&map, current, dir) {
                Some(x) => {
                    let k = (x.coord, x.dir, x.tiles_straight);
                    match visited.get(&k) {
                        Some(temp) if *temp < x.temp_drop => {}
                        _ => {
                            visited.insert(k, x.temp_drop);
                            queue.insert(x);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() {
    println!("part 1: {:?}", part1("inputs/17b"));
}
