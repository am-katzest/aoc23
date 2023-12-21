use itertools::Itertools;
use std::collections::HashMap;
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
    Map { tiles, start }
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
fn recursive_thing(m: &Map, max: usize, coord: Coord, t: usize, visited: &mut Visited) {
    for dir in DIRECTIONS {
        let t1 = t + 1;
        if t1 > max {
            continue;
        }

        let coord1 = step(*dir, coord);
        if m[coord1] == Tile::Blocked {
            continue;
        }

        let k = (coord1, mod2(t1));
        match visited.get_mut(&k) {
            // we are worse then previous one
            Some(previous) if *previous > t1 => {
                continue
            }

            _ => {
                visited.insert(k, t1);
            }
        }
        recursive_thing(m, max, coord1, t1, visited);
    }
}

fn part1(m: Map, age: usize) -> usize {
    println!("{:?}", m.start);
    let mut visited: Visited = HashMap::new();
    recursive_thing(&m, age, m.start, 0, &mut visited);
    let mut result = m.clone();
    for ((coord, m), _) in visited {
        if m == mod2(age) {
            result[coord] = Tile::Reachable;
        }
    }
    for row in result.tiles {
        for i in row {
            match i {
                Tile::Blocked => print!("#"),
                Tile::Ground => print!("."),
                Tile::Reachable => print!("O"),
            }
        }
        println!("", );
    }
    
    age
}

fn main() {
    println!("part 1: {:?}", part1(parse("inputs/21a"), 1));
    println!("part 1: {:?}", part1(parse("inputs/21a"), 2));
    println!("part 1: {:?}", part1(parse("inputs/21a"), 3));
    println!("part 1: {:?}", part1(parse("inputs/21a"), 4));
    println!("part 1: {:?}", part1(parse("inputs/21a"), 5));
    //println!("part 2: {:?}", part2(parse("inputs/10b"), Dir::Up));
}

#[cfg(test)]
mod tests {
    use crate::*;
}
