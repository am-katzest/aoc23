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
struct Nav {
    total_cost: usize, // Ord will check this first, rest doesn't *really* matter
    temp_drop: usize,
    dir: Dir,
    coord: Coord,
    tiles_straight: usize,
}
type Turning = (usize, usize);

fn create(m: &Map, parent: Nav, dir: Dir, (min, max): Turning) -> Option<Nav> {
    if dir == parent.dir && parent.tiles_straight >= max {
        return None;
    };
    if dir != parent.dir && parent.tiles_straight < min {
        return None;
    }
    let tiles_straight = if parent.dir == dir { parent.tiles_straight + 1 } else { 1 };
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

fn finished(n: Nav, m: &Map, (min, _): Turning) -> bool {
    let (x, y) = n.coord;
    m.size == (x + 1, y + 1) && n.tiles_straight >= min
}

fn solve(f: &str, t: Turning) -> usize {
    let map = parse(f);
    let initial = Nav {
        dir: Dir::Right, // should be able to pivot downwards
        coord: (0, 0),
        temp_drop: 0,
        total_cost: to_end((0, 0), &map),
        tiles_straight: 0,
    };
    let mut queue: BTreeSet<Nav> = BTreeSet::new();
    let mut visited: HashMap<(Coord, Dir, usize), usize> = HashMap::new();
    queue.insert(initial);
    queue.insert(Nav { dir: Dir::Down, ..initial });
    loop {
        let current = queue.pop_first().unwrap(); // we add them faster than we take them
        if finished(current, &map, t) {
            return current.temp_drop;
        }
        let d = current.dir;
        for dir in [d, clockwise(d), counterclockwise(d)] {
            match create(&map, current, dir, t) {
                Some(child) => {
                    let k = (child.coord, child.dir, child.tiles_straight);
                    match visited.get(&k) {
                        Some(previous_temp) if *previous_temp < child.temp_drop => {}
                        _ => {
                            visited.insert(k, child.temp_drop);
                            queue.insert(child);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() {
    println!("part 1: {:?}", solve("inputs/17b", (0, 3)));
    println!("part 2: {:?}", solve("inputs/17b", (4, 10)));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        assert_eq!(102, solve("inputs/17a", (0, 3)));
    }
    #[test]
    fn part2_test() {
        assert_eq!(94, solve("inputs/17a", (4, 10)));
        assert_eq!(71, solve("inputs/17c", (4, 10)));
    }
}
