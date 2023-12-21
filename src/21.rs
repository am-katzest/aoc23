use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Ground,
    Blocked,
    Reachable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Coord = (usize, usize);

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

//type Map = Vec<Vec<Tile>>;
#[derive(Clone, Debug)]
struct Map {
    start: Coord,
    tiles: Vec<Vec<Tile>>,
    size: Coord,
}

impl Index<Coord> for Map {
    type Output = Tile;
    fn index(&self, (x, y): Coord) -> &Tile {
        &self.tiles[y][x]
    }
}

impl IndexMut<Coord> for Map {
    fn index_mut(&mut self, (x, y): Coord) -> &mut Tile {
        &mut self.tiles[y][x]
    }
}

fn parse_tile(t: char) -> Tile {
    match t {
        '.' | 'S' => Tile::Ground,
        '#' => Tile::Blocked,
        _ => panic!("the fuck is {t}"),
    }
}

fn parse(f: &str) -> Map {
    let mut start = (0, 0);
    let tiles = read_to_string(f)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y)
                    };
                    parse_tile(c)
                })
                .collect_vec()
        })
        .collect_vec();
    let size = (tiles[0].len(), tiles.len());
    Map { tiles, start, size }
}

static DIRECTIONS: &[Dir] = &[Dir::Left, Dir::Right, Dir::Up, Dir::Down];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Mod2 {
    Odd,
    Even,
}

fn mod2(x: usize) -> Mod2 {
    match x % 2 {
        0 => Mod2::Even,
        _ => Mod2::Odd,
    }
}
type Visited = HashMap<(Coord, Mod2), usize>;
type Queue = VecDeque<(Coord, usize)>;
type Walker = (Coord, usize);

fn add_child(m: &Map, max: usize, (coord, t): Walker, visited: &mut Visited, queue: &mut Queue, dir: Dir) {
    let t1 = t + 1;
    if t1 > max {
        return;
    }
    let coord1 = match step(dir, coord, m) {
        None => return,
        Some(x) => x,
    };
    if m[coord1] == Tile::Blocked {
        return;
    }

    let k = (coord1, mod2(t1));

    match visited.get_mut(&k) {
        // we are worse then previous one
        Some(previous) if *previous >= t1 => {
            //println!("found previous",);
        }
        _ => {
            visited.insert(k, t1);
            queue.push_back((coord1, t1));
        }
    }
}

fn add_children(m: &Map, max: usize, coord: Coord, t: usize, visited: &mut Visited, queue: &mut Queue) {
    for dir in DIRECTIONS {
        add_child(m, max, (coord, t), visited, queue, *dir);
    }
}

fn part1(m: Map, age: usize) -> usize {
    println!("{:?}", m.start);
    let mut visited: Visited = HashMap::new();
    let mut queue: Queue = VecDeque::new();
    queue.push_back((m.start, 0));
    //
    loop {
        match queue.pop_front() {
            None => break,
            Some((c, a)) => {
                add_children(&m, age, c, a, &mut visited, &mut queue);
            }
        }
    }
    //
    let mut result = m.clone();
    for ((coord, m), _) in visited {
        if m == mod2(age) {
            result[coord] = Tile::Reachable;
        }
    }
    let mut res = 0;
    for row in result.tiles {
        for i in row {
            match i {
                Tile::Blocked => print!("#"),
                Tile::Ground => print!("."),
                Tile::Reachable => {
                    res += 1;
                    print!("O")
                }
            }
        }
        println!("",);
    }
    res
}

fn main() {
    //println!("part 1: {:?}", part1(parse("inputs/21a"), 6));
    println!("part 1: {:?}", part1(parse("inputs/21b"), 64));
    //println!("part 2: {:?}", part2(parse("inputs/10b"), Dir::Up));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test() {
        assert_eq!(2, part1(parse("inputs/21a"), 1));
        assert_eq!(16, part1(parse("inputs/21a"), 6));
        assert_eq!(18, part1(parse("inputs/21a"), 8));
    }
    use crate::*;
}
