use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Ground,
    Blocked,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

type Coord = (i32, i32);

fn step(d: Dir, (x, y): Coord) -> Coord {
    match d {
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
        Dir::Down => (x, y + 1),
        Dir::Up => (x, y - 1),
    }
}

//type Map = Vec<Vec<Tile>>;
#[derive(Clone, Debug)]
struct Map {
    start: Coord,
    tiles: Vec<Vec<Tile>>,
    size: Coord,
}

fn modulo(a: i32, b: i32) -> usize {
    (((a % b) + b) % b) as usize
}

impl Index<Coord> for Map {
    type Output = Tile;
    fn index(&self, (x, y): Coord) -> &Tile {
        &self.tiles[modulo(y, self.size.1)][modulo(x, self.size.0)]
    }
}

impl IndexMut<Coord> for Map {
    fn index_mut(&mut self, (x, y): Coord) -> &mut Tile {
        &mut self.tiles[modulo(y, self.size.1)][modulo(x, self.size.0)]
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
                        start = (x as i32, y as i32)
                    };
                    parse_tile(c)
                })
                .collect_vec()
        })
        .collect_vec();
    let size = (tiles[0].len() as i32, tiles.len() as i32);
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
    let coord1 = step(dir, coord);

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

fn part1(m: &Map, age: usize) -> usize {
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
    let mut res = 0;
    for ((coord, m), _) in visited {
        if m == mod2(age) {
            res += 1
        }
    }
    res
}

fn diff(m:&Map, age: usize ) -> usize {
    let exc = (age+1) * (age+1);
    exc - part1(m, age)
}
fn part2(m: &Map, age: usize) -> usize {
    let diameter = 131;
    let radius = 65;
    //for a in 60..70 {
    for a in 60..70 {
        println!("{a} -> {}", diff(&m, a));
    }
    3
}


fn main() {
    //println!("part 1: {:?}", part1(parse("inputs/21a"), 6));
    //println!("part 1: {:?}", part1(parse("inputs/21b"), 64));
    println!("part 2: {:?}", part2(&parse("inputs/21b"), 64*3+3));
    //println!("part 2: {:?}", part2(parse("inputs/10b"), Dir::Up));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test() {
        //        assert_eq!(2, part1(parse("inputs/21a"), 1));
        //        assert_eq!(16, part1(parse("inputs/21a"), 6));
        //        assert_eq!(18, part1(parse("inputs/21a"), 8));
    }
    use crate::*;
}
